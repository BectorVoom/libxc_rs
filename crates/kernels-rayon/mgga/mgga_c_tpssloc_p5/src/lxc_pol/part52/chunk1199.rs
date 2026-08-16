//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1199/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1199(t32726: f64, t539: f64, t31137: f64, t7691: f64, t6888: f64, t1375: f64, t1843: f64, t2016: f64, t26477: f64, t31106: f64, t31113: f64, t31189: f64, t32686: f64, t32690: f64, t32696: f64, t32700: f64, t32707: f64, t32708: f64, t568: f64, t6958: f64, t7750: f64) -> (f64, f64, f64) {
    let t32727 = t539 * t32726;
    let t32731 = t31137 * t7691;
    let t32733 = 0.3289868133696452873e-1_f64 * t6888 * t32731;
    let t32734 = 2.0_f64 * t1375 * t32686 - 6.0_f64 * t1375 * t32690 - t1843 * t31189 - 2.0_f64 * t2016 * t26477 + t32708 * t568 + t32727 * t568 - 2.0_f64 * t6958 * t7750 - t31106 - t31113 + t32696 - t32700 + t32707 - t32733;
    (t32727, t32731, t32734)
}
