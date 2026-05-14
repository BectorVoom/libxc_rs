//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 694/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk694<F: Float>(t7515: F, t7519: F, t7539: F, t7545: F, t7549: F, t7557: F, t7601: F, t7611: F, t7631: F, t7638: F, t7640: F, t7671: F, t7673: F, t7677: F, t7696: F, t7717: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8184 = 0.85748036236139473944e-3 * t7515;
    let t8185 = 0.12579236915841660827e-2 * t7519;
    let t8190 = 0.62896184579208304138e-3 * t7539;
    let t8192 = 0.31448092289604152069e-3 * t7545;
    let t8193 = 0.41930789719472202758e-3 * t7549;
    let t8195 = 0.62896184579208304138e-3 * t7557;
    let t8205 = 0.13073958333333333333e0 * t7601;
    let t8209 = 0.21437009059034868486e-3 * t7611;
    let t8219 = 0.37737710747524982482e-2 * t7631;
    let t8220 = 0.27953859812981468505e-2 * t7638;
    let t8221 = 0.25724410870841842184e-2 * t7640;
    let t8232 = 0.42874018118069736972e-3 * t7671;
    let t8233 = 13.0 / 144.0 * t7673;
    let t8235 = 0.25724410870841842184e-2 * t7677;
    let t8240 = 0.37737710747524982482e-2 * t7696;
    let t8247 = 0.42874018118069736972e-3 * t7717;
    (t8184, t8185, t8190, t8192, t8193, t8195, t8205, t8209, t8219, t8220, t8221, t8232, t8233, t8235, t8240, t8247)
}
