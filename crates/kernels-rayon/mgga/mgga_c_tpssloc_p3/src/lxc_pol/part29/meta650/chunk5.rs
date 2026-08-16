//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2174/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2174(t22574: f64, t56194: f64, t8643: f64, t12461: f64, t6995: f64, t26161: f64, t26163: f64, t22581: f64, t7685: f64, t24987: f64, t7000: f64, t25985: f64, t6876: f64) -> (f64, f64, f64, f64, f64) {
    let t90029 = 6.0_f64 * t22574 * t8643 * t56194;
    let t90031 = t6995 * t12461;
    let t90034 = 4.0_f64 * t26161 * t90031 * t26163;
    let t90036 = 2.0_f64 * t7685 * t22581;
    let t90038 = 2.0_f64 * t24987 * t7000;
    let t90040 = 6.0_f64 * t6876 * t25985;
    (t90029, t90034, t90036, t90038, t90040)
}
