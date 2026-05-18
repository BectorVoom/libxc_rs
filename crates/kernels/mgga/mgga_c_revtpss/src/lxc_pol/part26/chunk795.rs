//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 795/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk795<F: Float>(t2367: F, t625: F, t654: F, t2340: F, t665: F, t2339: F, t2366: F, t2269: F, t98: F, t99: F, t2350: F, t658: F, tau0: F) -> (F, F, F, F, F, F, F, F) {
    let t10206 = t625 * t2367;
    let t10207 = t654 * t654;
    let t10208 = F::new(1.0) / t10207;
    let t10209 = t2340 * t665;
    let t10210 = t10208 * t10209;
    let t10213 = t2339 * t665;
    let t10214 = t10213 * t2366;
    let t10217 = tau0 * t2269;
    let t10226 = t99 * t98;
    let t10227 = F::new(1.0) / t10226;
    let t10228 = t2350 * t658;
    (t10206, t10208, t10209, t10210, t10214, t10217, t10227, t10228)
}
