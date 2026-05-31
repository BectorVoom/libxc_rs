//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1196/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1196<F: Float>(t1165: F, t34248: F, t5532: F, t7564: F, t5537: F, t8600: F, t30219: F, t9670: F, t36274: F, t36284: F, t36287: F, t36293: F, t36294: F, t36300: F, t36303: F, t37940: F, t40465: F, t40467: F, t40469: F, t40472: F, t40474: F, t40477: F) -> F {
    let t40481 = t7564 * t1165 * t34248 * t5532;
    let t40485 = t7564 * t1165 * t8600 * t5537;
    let t40487 = t30219 * t9670;
    let t40489 = -F::cast_from(0.12579236915841660827e-2_f64) * t40465 + F::cast_from(0.17149607247227894789e-2_f64) * t40467 + F::cast_from(0.17149607247227894789e-2_f64) * t40469 + t36274 + t36284 - t36287 - t37940 + t36293 - F::cast_from(0.27953859812981468504e-2_f64) * t36294 + t36300 + t36303 + F::cast_from(0.17149607247227894789e-2_f64) * t40472 + F::cast_from(0.42874018118069736972e-3_f64) * t40474 + t40477 / F::cast_from(16.0_f64) + F::cast_from(0.94344276868812456205e-2_f64) * t40481 - F::cast_from(0.37737710747524982482e-2_f64) * t40485 + F::cast_from(0.21437009059034868486e-2_f64) * t40487;
    t40489
}
