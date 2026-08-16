//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1080/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1080(t10969: f64, t32411: f64, t34707: f64, t8392: f64, t1852: f64, t3255: f64, t7281: f64, t1882: f64, t34663: f64, t6547: f64, t91496: f64, t11593: f64, t11837: f64, t137812: f64, t137814: f64, t144657: f64, t1901: f64, t1909: f64, t3052: f64, t3238: f64, t32488: f64, t32630: f64, t34511: f64, t432: f64, t446: f64, t452: f64, t488: f64, t6534: f64, t7229: f64, t83: f64, t93636: f64) -> (f64, f64, f64, f64, f64) {
    let t146143 = t10969 * t32411;
    let t146150 = t8392 * t34707;
    let t146171 = t1852 * t7281 * t3255;
    let t146175 = t1882 * t34663;
    let t146182 = t91496 * t6547;
    let t146201 = -2.0_f64 / 3.0_f64 * t446 * t83 * t144657 + 4.0_f64 / 9.0_f64 * t137812 - 2.0_f64 / 27.0_f64 * t137814 + 2.0_f64 / 3.0_f64 * t446 * t83 * t146171 - 2.0_f64 / 9.0_f64 * t146175 + t446 * t452 * t488 * t34511 * t432 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t83 * t146182 - 4.0_f64 / 9.0_f64 * t11593 * t1909 * t32488 * t3052 + 2.0_f64 / 9.0_f64 * t1901 * t93636 * t6534 + t446 * t452 * t3238 * t32630 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t452 * t11837 * t7229;
    (t146143, t146150, t146171, t146182, t146201)
}
