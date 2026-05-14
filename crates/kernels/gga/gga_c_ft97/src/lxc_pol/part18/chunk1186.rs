//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1186/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1186<F: Float>(t12486: F, t173: F, t25753: F, t25754: F, t34434: F, t11982: F, t1737: F, t5570: F, t11437: F, t8120: F, t100580: F, t100669: F, t101004: F, t11213: F, t1300: F, t1669: F, t1701: F, t1712: F, t2035: F, t22652: F, t22696: F, t22834: F, t25625: F, t25626: F, t25631: F, t25637: F, t25640: F, t25676: F, t25708: F, t25755: F, t3019: F, t3061: F, t3099: F, t391: F, t423: F, t5790: F, t6428: F, t6431: F, t7867: F, t7983: F, t9: F, t920: F, t92339: F, t92399: F, t92873: F, t929: F, t92926: F, t93015: F, t93092: F, t93102: F, t93106: F, t93168: F, t93169: F, t938: F) -> (F, F, F) {
    let t101387 = t25753 * t25754 * t173 * t12486;
    let t101402 = t25754 * t34434;
    let t101406 = t5570 * t1737 * t11982;
    let t101410 = t5570 * t8120 * t11437;
    let t101437 = 0.1054015240332537869e-3 * t7867 * t2035 * t5790 * t3099 + 0.77462893625097599762e-3 * t101004 * t3061 + 0.27620809331261011348e-4 * t25753 * t9 * t391 * t25755 - 0.34526011664076264184e-5 * t101387 - 0.23254900946437792e-1 * t93092 * t6428 - 0.46509801892875584e-1 * t22834 * t25631 + 0.51074886703703703704e-1 * t93168 * t93169 * t100580 * t100669 - 0.38306165027777777778e-1 * t92873 * t5570 * t423 * t920 * t1712 + 0.10357803499222879255e-4 * t93015 * t101402 - 0.85124811172839506173e-2 * t25708 * t101406 - 0.19862455940329218107e-1 * t25708 * t101410 - 0.2370952259137005195e-1 * t1300 * t1701 * t22652 * t3099 + 0.13519760450715832853e-3 * t11213 * t25626 + 0.27039520901431665706e-3 * t3019 * t93102 * t25625 + 0.27039520901431665706e-3 * t3019 * t7983 * t92339 * t929 - 2.0 * t93106 * t6431 - 4.0 * t22696 * t25637 - 4.0 * t22696 * t25640 - 2.0 * t1669 * t92926 * t938 - 0.2370952259137005195e-1 * t92399 * t25676;
    (t101406, t101410, t101437)
}
