//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1269/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1269<F: Float>(t6864: F, t9918: F, t1353: F, t6816: F, t4012: F, t828: F, t3930: F, t6876: F, t1883: F, t5627: F, t13783: F, t13926: F, t6869: F, t13789: F, t14038: F, t14040: F, t14042: F, t14043: F, t14049: F, t14053: F, t14057: F, t1410: F, t3934: F, t9977: F) -> (F,) {
    let t22285 = t9918 * t6864;
    let t22287 = t6816 * t1353;
    let t22289 = t4012 * t828 * t22287;
    let t22292 = t3930 * t6876;
    let t22294 = t1883 * t5627;
    let t22295 = t13783 * t22294;
    let t22298 = t13926 * t6869;
    let t22299 = t13789 * t22298;
    let t22304 = -0.20007875121765877254e-2 * t22285 + 0.42874018118069736972e-2 * t1410 * t22289 + 0.10003937560882938627e-2 * t22292 - 0.85748036236139473945e-2 * t3934 * t22295 + 0.17149607247227894789e-2 * t3934 * t22299 - t14038 - t14040 + t14042 + 0.27104001498285508386e-3 * t14043 - t14049 + t14053 - t14057 + 0.13552000749142754193e-3 * t9977;
    (t22304,)
}
