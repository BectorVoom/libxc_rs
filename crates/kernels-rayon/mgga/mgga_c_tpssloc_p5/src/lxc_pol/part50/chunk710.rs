//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 710/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk710(t1030: f64, t1940: f64, t354: f64, t1036: f64, t1942: f64, t1039: f64) -> (f64, f64, f64, f64, f64) {
    let t6758 = t1940 * t1030;
    let t6759 = t354 * t6758;
    let t6763 = t1942 * t1036 / 2304.0_f64;
    let t6764 = t1940 * t1039;
    let t6765 = t354 * t6764;
    (t6758, t6759, t6763, t6764, t6765)
}
