//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1029/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1029(t115743: f64, t115748: f64, t115750: f64, t115752: f64, t115754: f64, t115757: f64, t115766: f64, t115771: f64, t115773: f64, t115777: f64, t115781: f64, t116135: f64, t116304: f64, t12734: f64, t2096: f64, t2165: f64, t2314: f64, t23933: f64, t24008: f64, t24433: f64, t32318: f64, t7040: f64, t7266: f64, t7408: f64, t8835: f64) -> f64 {
    let t117648 = -6.0_f64 * t116135 * t24433 + t116304 * t2096 - 4.0_f64 * t12734 * t8835 - t2165 * t24008 - 4.0_f64 * t2314 * t32318 - 4.0_f64 * t23933 * t7266 - 2.0_f64 * t7040 * t7408 - t115743 - t115748 + t115750 - t115752 - t115754 - t115757 + t115766 - t115771 - t115773 + t115777 - t115781;
    t117648
}
