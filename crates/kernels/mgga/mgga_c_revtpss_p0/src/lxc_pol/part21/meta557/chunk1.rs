//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2248/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2248<F: Float>(t1222: F, t12855: F, t12910: F, t13069: F, t17437: F, t17438: F, t17444: F, t17447: F, t17448: F, t17453: F, t17456: F, t17461: F, t17464: F, t17467: F, t1797: F, t3631: F, t3674: F) -> F {
    let t17470 = -t17437 - F::cast_from(0.22866142996303859718e-2_f64) * t17438 * t3674 + F::cast_from(0.21437009059034868486e-3_f64) * t13069 * t1797 + t17444 - t17447 - F::cast_from(0.28582678745379824648e-3_f64) * t17448 * t3631 - t17453 - F::cast_from(0.85748036236139473944e-3_f64) * t12855 * t17456 + F::cast_from(0.85748036236139473944e-3_f64) * t12910 * t17461 + t1222 * t17464 / F::new(108.0) + t1222 * t17467 / F::new(216.0);
    t17470
}
