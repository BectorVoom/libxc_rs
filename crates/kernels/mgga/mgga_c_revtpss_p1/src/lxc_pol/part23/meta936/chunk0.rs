//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3076/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3076<F: Float>(t1196: F, t20890: F, t58647: F, t24473: F, t3531: F, t24764: F, t5206: F, t20400: F, t5207: F, t20692: F, t29322: F, t5023: F, t5501: F, t73252: F, t81322: F, t81326: F, t81328: F, t81330: F, t81333: F) -> (F, F, F, F, F) {
    let t81336 = F::cast_from(0.30762056574649219974e4_f64) * t1196 * t20890 * t58647;
    let t81338 = F::cast_from(0.51947577317044391277e2_f64) * t3531 * t24473;
    let t81341 = F::cast_from(0.6233709278045326953e3_f64) * t1196 * t24764 * t5206;
    let t81343 = F::cast_from(0.51947577317044391276e2_f64) * t20400 * t5207;
    let t81350 = -F::new(3.0) * t20692 * t5023 * t5501 + F::new(6.0) * t29322 * t5023 * t73252 - t81322 - t81326 + t81328 + t81330 + t81333 - t81336 - t81338 - t81341 - t81343;
    (t81336, t81338, t81341, t81343, t81350)
}
