//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1275/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1275(t24352: f64, t2920: f64, t35894: f64, t10105: f64, t3724: f64, t10343: f64, t11695: f64, t12333: f64, t12345: f64, t12328: f64, t12340: f64, t12343: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36040 = t2920 * t24352 * t35894;
    let t36042 = t10105 * t3724;
    let t36044 = t10343 * t11695;
    let t37331 = 8.0_f64 * t12333;
    let t37332 = 2.0_f64 * t12345;
    let t37333 = 2.0_f64 * t12328;
    let t37334 = 12.0_f64 * t12340;
    let t37335 = 8.0_f64 * t12343;
    (t36040, t36042, t36044, t37331, t37332, t37333, t37334, t37335)
}
