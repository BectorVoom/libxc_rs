//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 919/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk919<F: Float>(t11479: F, t11480: F, t18919: F, t18924: F, t18934: F, t19002: F, t19004: F, t19009: F, t23521: F, t23523: F, t23536: F, t23538: F, t23541: F, t23543: F, t23680: F, t964: F, t973: F) -> (F, F) {
    let t23693 = 0.20128333333333333333e0 * t18919 - 0.60385000000000000001e0 * t18924 + 0.30192500000000000001e0 * t18934 - t11479 - t11480 + 0.5519e-1 * t19002 - 0.33114e0 * t19004 + 0.16557e0 * t19009 - 0.3883875e1 * t23521 + 0.247573125e0 * t23523 + 0.258925e1 * t23536 + 0.16504875e0 * t23538 + 0.19419375e1 * t23541 - 0.412621875e-1 * t23543;
    let t23694 = t23680 + t23693;
    let t23696 = t964 * t23694 * t973;
    (t23694, t23696)
}
