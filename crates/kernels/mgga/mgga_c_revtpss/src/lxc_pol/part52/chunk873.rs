//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 873/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk873<F: Float>(t27246: F, t27251: F, t27254: F, t27256: F, t25224: F, t25230: F, t25236: F, t25279: F, t26457: F, t26462: F, t26468: F, t26471: F, t27244: F, t27249: F, t27262: F, t28331: F) -> (F,) {
    let t28333 = 7.0 / 72.0 * t27246;
    let t28335 = 0.2032800112371413129e-3 * t27251;
    let t28336 = 0.28582678745379824648e-4 * t27254;
    let t28337 = 0.16006300097412701803e-1 * t27256;
    let t28339 = t25279 - t26471 - t27244 / 24.0 + t28333 - t25236 + t26457 + t26468 - 0.34299214494455789578e-2 * t27249 - t28335 + t28336 + t28337 + t26462 + t25224 + t25230 + 0.17149607247227894789e-2 * t27262;
    let t28340 = t28331 + t28339;
    (t28340,)
}
