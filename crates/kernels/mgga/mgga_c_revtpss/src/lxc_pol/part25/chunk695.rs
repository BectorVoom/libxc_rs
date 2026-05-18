//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 695/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk695<F: Float>(t116: F, t2327: F, t117: F, t2371: F, t1459: F, t1461: F, t4158: F, t572: F, t573: F, t670: F, t94: F) -> (F, F, F, F) {
    let t4162 = t116 * t2327;
    let t4165 = t117 * t2371;
    let t4168 = F::new(6.0) * t1459 * t1461 + t4158 * t573 + F::new(6.0) * t4162 * t572 + F::new(3.0) * t4165 * t572;
    let t4254 = t94 * t670;
    (t4162, t4165, t4168, t4254)
}
