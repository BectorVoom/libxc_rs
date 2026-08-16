//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 803/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk803(t4021: f64, t79: f64, t72: f64, t1410: f64, t2235: f64, t3961: f64, t605: f64, t3967: f64, t1433: f64, t645: f64, t1458: f64, t649: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26066 = t79 * t4021;
    let t26067 = t72 * t26066;
    let t26070 = t2235 * t1410;
    let t26073 = t605 * t3961;
    let t26076 = t605 * t3967;
    let t26090 = t72 * t1433 * t645;
    let t26114 = t649 * t1458;
    (t26067, t26070, t26073, t26076, t26090, t26114)
}
