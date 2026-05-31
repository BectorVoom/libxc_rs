//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2025/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2025<F: Float>(t14386: F, t707: F, t189: F, t4186: F, t606: F, t4401: F, t10579: F, t2411: F, t4537: F, t10446: F, t1469: F, t2375: F) -> (F, F, F, F, F, F, F) {
    let t14388 = F::cast_from(8.0_f64) * t14386 * t707;
    let t14389 = t189 * t4186;
    let t14390 = t14389 * t606;
    let t14392 = F::cast_from(24.0_f64) * t4401 * t14390;
    let t14396 = F::cast_from(0.21687162600603479684e-1_f64) * t10579;
    let t14397 = t4537 * t2411;
    let t14401 = t10446 * t1469;
    let t14404 = t2375 * t4186;
    (t14388, t14390, t14392, t14396, t14397, t14401, t14404)
}
