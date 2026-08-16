//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2226/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2226<F: Float>(t16219: F, t7111: F, t139: F, t27526: F, t3252: F, t4574: F, t1014: F, t4579: F, t15130: F, t15135: F, t15140: F, t15145: F, t15149: F, t15154: F, t15651: F, t1665: F, t25490: F, t27527: F, t27531: F, t4854: F, t53321: F, t7117: F, t93736: F) -> F {
    let t100365 = t7111 * t16219;
    let t100370 = t27526 * t139 * t3252 * t4574 / F::cast_from(324.0_f64);
    let t100398 = t27526 * t139 * t1014 * t4579 / F::cast_from(216.0_f64);
    let t100399 = -t100365 / F::cast_from(1296.0_f64) + t100370 - t27526 * t27527 * t15145 / F::cast_from(72.0_f64) - t27526 * t27527 * t15149 / F::cast_from(144.0_f64) - t27526 * t27531 * t15154 / F::cast_from(36.0_f64) + t27526 * t27531 * t15130 / F::cast_from(108.0_f64) + t27526 * t27531 * t15135 / F::cast_from(216.0_f64) + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t27526 * t53321 * t15140 - F::cast_from(0.85748036236139473944e-3_f64) * t25490 * t4854 - F::cast_from(0.42874018118069736972e-3_f64) * t7117 * t15651 - F::cast_from(0.42874018118069736972e-3_f64) * t93736 * t1665 - t100398;
    t100399
}
