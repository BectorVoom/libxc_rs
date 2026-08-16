//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 842/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk842(t3406: f64, t7115: f64, t9921: f64, t10029: f64, t2598: f64, t3404: f64, t1038: f64, t2232: f64, t3403: f64, t3413: f64, t829: f64, t3438: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10030 = t7115 * t3406;
    let t10031 = t9921 * t10030;
    let t10032 = t10029 * t10031;
    let t10034 = t3404 * t2598;
    let t10035 = t1038 * t2232;
    let t10036 = t10034 * t10035;
    let t10037 = t3403 * t10036;
    let t10039 = t2598 * t3413;
    let t10040 = t829 * t10039;
    let t10041 = t3438 * t10040;
    (t10031, t10032, t10036, t10037, t10039, t10041)
}
