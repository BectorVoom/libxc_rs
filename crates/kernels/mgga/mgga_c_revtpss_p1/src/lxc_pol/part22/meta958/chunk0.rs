//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3216/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3216<F: Float>(t49864: F, t10605: F, t18539: F, t49866: F, t39423: F, t39425: F, t39433: F, t39436: F, t14365: F, t18865: F, t2403: F, t39419: F, t39422: F, t39429: F, t39432: F) -> (F, F, F, F, F, F, F, F) {
    let t61019 = F::new(2.0) * t49864;
    let t61020 = t10605 * t18539;
    let t61021 = F::new(24.0) * t61020;
    let t61022 = F::cast_from(0.20508037716432813315e4_f64) * t49866;
    let t61026 = F::cast_from(0.43374325201206959368e-1_f64) * t39423;
    let t61027 = F::cast_from(0.65061487801810439052e-1_f64) * t39425;
    let t61028 = F::cast_from(0.96319466275353142156e0_f64) * t39433;
    let t61029 = F::cast_from(0.10843581300301739842e-1_f64) * t39436;
    let t61030 = -F::new(6.0) * t14365 * t18865 * t2403 - t39419 - t39422 - t39429 - t39432 + t61019 + t61021 - t61022 - t61026 - t61027 + t61028 + t61029;
    (t61019, t61021, t61022, t61026, t61027, t61028, t61029, t61030)
}
