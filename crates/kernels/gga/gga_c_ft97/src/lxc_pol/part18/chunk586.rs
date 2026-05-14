//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 586/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk586<F: Float>(t424: F, t626: F, t419: F, t173: F, t1747: F, t1738: F, t1570: F, t23: F, t7763: F, t1675: F, t67: F, t9: F, t1725: F, t1732: F, t1693: F, t10: F, t3050: F, t83: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8109 = t626 * t424;
    let t8110 = t419 * t8109;
    let t8112 = t173 * t1747;
    let t8113 = t419 * t8112;
    let t8115 = t173 * t1738;
    let t8116 = t419 * t8115;
    let t8119 = 1.0 / t23 / t1570;
    let t8120 = t8119 * t7763;
    let t8130 = t9 * t67 * t1675;
    let t8133 = t1725 * t1732;
    let t8155 = t1693 * t1693;
    let t8189 = t10 * t3050 * t83;
    (t8110, t8113, t8116, t8119, t8120, t8130, t8133, t8155, t8189)
}
