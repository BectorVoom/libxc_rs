//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 815/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk815<F: Float>(t10208: F, t10209: F, t2339: F, t665: F, t2366: F, t2269: F, t98: F, t99: F, t2350: F, t658: F, t2349: F, t2256: F, t9343: F, t100: F, t106: F, t107: F, tau0: F) -> (F, F, F, F, F, F, F, F) {
    let t10210 = t10208 * t10209;
    let t10213 = t2339 * t665;
    let t10214 = t10213 * t2366;
    let t10217 = tau0 * t2269;
    let t10226 = t99 * t98;
    let t10227 = 1.0 / t10226;
    let t10228 = t2350 * t658;
    let t10229 = t10227 * t10228;
    let t10232 = t2349 * t658;
    let t10233 = t10232 * t2256;
    let t10236 = 3.0 * t9343;
    let t10237 = t100 * t10236;
    let t10240 = t107 * t106;
    (t10210, t10214, t10217, t10229, t10233, t10236, t10237, t10240)
}
