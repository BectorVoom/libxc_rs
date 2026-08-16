//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1084/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1084(t1985: f64, t6907: f64, t80707: f64, t22675: f64, t22724: f64, t22666: f64, t22916: f64, t6888: f64, t22716: f64, t6903: f64, t22662: f64, t22674: f64, t6897: f64) -> (f64, f64, f64, f64, f64) {
    let t80709 = t1985 * t80707 * t6907;
    let t80711 = t22724 * t22675;
    let t80714 = t6888 * t22666 * t22916;
    let t80722 = t22716 * t6903;
    let t80725 = t6897 * t22674 * t22662;
    (t80709, t80711, t80714, t80722, t80725)
}
