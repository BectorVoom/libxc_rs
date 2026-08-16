//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 892/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk892(t11135: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t11150: f64, t11156: f64, t11161: f64, t11165: f64, t11170: f64, t11174: f64, t423: f64) -> f64 {
    let t11459 = 0.55403703703703703703e-1_f64 * t11135;
    let t11470 = -t11459 + 0.23744444444444444444e-1_f64 * t11137 + 0.11872222222222222222e-1_f64 * t11139 - 0.35616666666666666666e-1_f64 * t11141 - 0.17808333333333333333e-1_f64 * t11143 + 0.19787037037037037037e-1_f64 * t11150 - 0.71233333333333333332e-1_f64 * t11156 - 0.35616666666666666666e-1_f64 * t11161 + 0.10685e0_f64 * t11165 + 0.10685e0_f64 * t11170 + 0.17808333333333333333e-1_f64 * t11174;
    let t11472 = 0.621814e-1_f64 * t11470 * t423;
    t11472
}
