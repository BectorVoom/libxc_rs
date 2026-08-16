//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 585/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk585(t3439: f64, t829: f64, t3438: f64, t311: f64, t896: f64, t315: f64, t3171: f64, t2913: f64, t2916: f64, t2926: f64, t2930: f64, t2934: f64, t2939: f64, t2943: f64, t2946: f64, t2949: f64, t2955: f64, t2959: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3440 = t829 * t3439;
    let t3441 = t3438 * t3440;
    let t3443 = t311 * t896;
    let t3444 = t3171 * t315;
    let t3445 = t3443 * t3444;
    let t3477 = -0.60736713313768998073e-4_f64 * t2913 - 0.20245571104589666024e-4_f64 * t2916 + 0.29524791194193262952e-5_f64 * t2926 - 0.60736713313768998073e-4_f64 * t2930 - 0.43449121406768801913e-4_f64 * t2934 + 0.43449121406768801913e-4_f64 * t2939 + 0.43449121406768801913e-5_f64 * t2943 - 0.77252537861234929801e-5_f64 * t2946 - 0.43449121406768801913e-4_f64 * t2949 - 0.12672660410307567225e-4_f64 * t2955 + 0.43449121406768801913e-4_f64 * t2959;
    (t3440, t3441, t3443, t3444, t3445, t3477)
}
