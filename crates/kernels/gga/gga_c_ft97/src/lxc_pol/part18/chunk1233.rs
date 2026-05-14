//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1233/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1233<F: Float>(t25598: F, t8466: F, t6471: F, t8232: F, t110: F, t11496: F, t11556: F, t11593: F, t11604: F, t12008: F, t12034: F, t1564: F, t1820: F, t1825: F, t1901: F, t23018: F, t23250: F, t23323: F, t26410: F, t26440: F, t26445: F, t3189: F, t3255: F, t3281: F, t38711: F, t446: F, t452: F, t488: F, t5617: F, t5691: F, t5710: F, t59631: F, t6454: F, t83: F, t8411: F, t91523: F, t93636: F, t986: F) -> (F, F) {
    let t102585 = t8466 * t25598;
    let t102599 = t8232 * t6471;
    let t102604 = 2.0 / 3.0 * t446 * t452 * t5710 * t11496 + 4.0 / 9.0 * t1901 * t93636 * t3189 + 8.0 / 27.0 * t11593 * t11556 * t26440 * t11604 - 2.0 / 9.0 * t1901 * t38711 * t26445 - 2.0 * t446 * t8411 * t986 * t23018 + t446 * t452 * t488 * t6454 * t1820 / 3.0 + 2.0 / 3.0 * t446 * t452 * t1825 * t26410 + 2.0 / 3.0 * t446 * t452 * t488 * t5617 * t3255 - 2.0 / 27.0 * t91523 + 4.0 / 3.0 * t446 * t83 * t102585 + 2.0 / 9.0 * t1901 * t23323 * t12034 + t1901 * t23323 * t12008 / 9.0 + 2.0 / 9.0 * t3281 * t1564 * t110 * t5691 + 8.0 / 27.0 * t102599 - 4.0 / 3.0 * t1901 * t59631 * t23250;
    (t102585, t102604)
}
