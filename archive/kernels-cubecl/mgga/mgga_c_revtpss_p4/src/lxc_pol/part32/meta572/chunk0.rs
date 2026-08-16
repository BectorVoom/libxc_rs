//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1896/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1896<F: Float>(t2028: F, t28911: F, t25894: F, t97680: F, t25875: F, t96236: F, t97688: F, t26304: F, t97705: F, t96187: F, t97685: F, t136: F, t2457: F, t8103: F) -> (F, F, F, F, F, F, F, F) {
    let t102078 = t2028 * t28911;
    let t102081 = F::cast_from(0.28912093960683998208e-1_f64) * t25894 * t102078 * t97680;
    let t102084 = F::cast_from(0.51405703062096148812e-1_f64) * t25875 * t102078 * t97680;
    let t102086 = F::cast_from(0.51405703062096148812e-1_f64) * t96236 * t97688;
    let t102087 = t2028 * t26304;
    let t102090 = F::cast_from(0.14456046980341999104e-1_f64) * t25894 * t102087 * t97705;
    let t102093 = F::cast_from(0.25702851531048074406e-1_f64) * t25875 * t102087 * t97705;
    let t102096 = F::cast_from(0.28912093960683998208e-1_f64) * t96187 * t97685;
    let t102098 = F::cast_from(0.51405703062096148812e-1_f64) * t96236 * t97685;
    let t102100 = t8103 * t136 * t2457;
    (t102081, t102084, t102086, t102090, t102093, t102096, t102098, t102100)
}
