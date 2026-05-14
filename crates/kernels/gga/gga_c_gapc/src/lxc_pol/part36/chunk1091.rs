//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1091/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1091<F: Float>(t24352: F, t2920: F, t35894: F, t10105: F, t3724: F, t10343: F, t11695: F, t12333: F, t12345: F, t12328: F, t12340: F, t12343: F, t12331: F, t12434: F, t10526: F, t3537: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t36040 = t2920 * t24352 * t35894;
    let t36042 = t10105 * t3724;
    let t36044 = t10343 * t11695;
    let t37331 = 8.0 * t12333;
    let t37332 = 2.0 * t12345;
    let t37333 = 2.0 * t12328;
    let t37334 = 12.0 * t12340;
    let t37335 = 8.0 * t12343;
    let t37336 = 4.0 * t12331;
    let t37337 = 2.0 * t12434;
    let t37339 = 4.0 * t10526 * t3537;
    (t36040, t36042, t36044, t37331, t37332, t37333, t37334, t37335, t37336, t37337, t37339)
}
