//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1285/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1285<F: Float>(t209: F, t37370: F, t37384: F, t37399: F, t37414: F, t37430: F, t37444: F, t37459: F, t37474: F, t12464: F, t883: F, t11039: F, t1125: F, t13296: F, t2469: F, t2470: F, t2822: F, t37339: F, t37342: F, t37344: F, t37346: F, t37349: F, t37352: F, t37354: F, t3883: F, t3897: F, t7053: F, t7063: F, t972: F) -> (F, F) {
    let t37478 = (t37370 + t37384 + t37399 + t37414 + t37430 + t37444 + t37459 + t37474) * t209;
    let t37484 = t12464 * t883;
    let t37496 = F::cast_from(4.0_f64) * t11039 * t1125 * t2469 + F::cast_from(24.0_f64) * t13296 * t2470 * t3883 + F::cast_from(2.0_f64) * t2469 * t2822 * t3897 - F::cast_from(6.0_f64) * t2470 * t3897 * t7063 - F::cast_from(2.0_f64) * t37484 * t972 - t3897 * t7053 + t37339 + t37342 + t37344 - t37346 - t37349 + t37352 - t37354;
    (t37478, t37496)
}
