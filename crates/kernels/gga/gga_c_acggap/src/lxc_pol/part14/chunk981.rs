//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 981/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk981<F: Float>(t31773: F, t9660: F, t7447: F, t9730: F, t30569: F, t30570: F, t30577: F, t30582: F, t34422: F, t34430: F, t37093: F, t39299: F, t39302: F, t39305: F, t39308: F, t39311: F, t39314: F, t39318: F, t39320: F, t39322: F) -> (F,) {
    let t39324 = t31773 * t9660;
    let t39326 = t7447 * t9730;
    let t39328 = -t34422 - 5.0 / 32.0 * t39299 - t39302 / 32.0 + t39305 / 16.0 + t39308 / 64.0 + t39311 / 64.0 - t39314 / 64.0 - t30569 - t34430 - 0.94344276868812456205e-2 * t30570 + t30577 + 0.62896184579208304134e-3 * t30582 - t37093 + t39318 / 48.0 - 0.84046875e-1 * t39320 + 0.84046875e-1 * t39322 + 0.16809375e0 * t39324 - 11.0 / 192.0 * t39326;
    (t39328,)
}
