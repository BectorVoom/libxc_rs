//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1293/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1293(t36009: f64, t36011: f64, t36013: f64, t36017: f64, t36020: f64, t36022: f64, t36025: f64, t36028: f64, t36030: f64, t36034: f64, t36037: f64, t36040: f64, t36042: f64, t36044: f64) -> f64 {
    let t37614 = -0.65659062294300875668e-4_f64 * t36009 + 0.28452260327530379456e-3_f64 * t36011 + 0.28452260327530379456e-3_f64 * t36013 + 0.58714905980103539484e-5_f64 * t36017 - 0.32829531147150437834e-4_f64 * t36020 + 0.32829531147150437834e-4_f64 * t36022 + 0.93943849568165663176e-5_f64 * t36025 + 0.65659062294300875668e-4_f64 * t36028 - 0.43840463131810642815e-4_f64 * t36030 + 0.65659062294300875668e-4_f64 * t36034 - 0.32829531147150437834e-4_f64 * t36037 - 0.45596571037708941436e-6_f64 * t36040 + 0.75872694206747678549e-3_f64 * t36042 - 0.43316742485823494364e-5_f64 * t36044;
    t37614
}
