//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1285/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1285(t209: f64, t37370: f64, t37384: f64, t37399: f64, t37414: f64, t37430: f64, t37444: f64, t37459: f64, t37474: f64, t12464: f64, t883: f64, t11039: f64, t1125: f64, t13296: f64, t2469: f64, t2470: f64, t2822: f64, t37339: f64, t37342: f64, t37344: f64, t37346: f64, t37349: f64, t37352: f64, t37354: f64, t3883: f64, t3897: f64, t7053: f64, t7063: f64, t972: f64) -> (f64, f64) {
    let t37478 = (t37370 + t37384 + t37399 + t37414 + t37430 + t37444 + t37459 + t37474) * t209;
    let t37484 = t12464 * t883;
    let t37496 = 4.0_f64 * t11039 * t1125 * t2469 + 24.0_f64 * t13296 * t2470 * t3883 + 2.0_f64 * t2469 * t2822 * t3897 - 6.0_f64 * t2470 * t3897 * t7063 - 2.0_f64 * t37484 * t972 - t3897 * t7053 + t37339 + t37342 + t37344 - t37346 - t37349 + t37352 - t37354;
    (t37478, t37496)
}
