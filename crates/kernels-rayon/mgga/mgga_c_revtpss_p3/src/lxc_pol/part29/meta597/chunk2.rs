//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2017/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2017(t99099: f64, t99102: f64, t99113: f64, t93067: f64, t93069: f64, t93073: f64, t93077: f64, t93080: f64, t93084: f64, t93086: f64, t93088: f64, t93091: f64, t93095: f64) -> f64 {
    let t103336 = 7.0_f64 / 36.0_f64 * t99099;
    let t103337 = 7.0_f64 / 12.0_f64 * t99102;
    let t103347 = 0.18071592998981862717e-4_f64 * t99113;
    let t103349 = t103336 - t103337 - 0.18140473443734395377e0_f64 * t93067 + 0.16006300097412701803e-1_f64 * t93069 + 0.43366402397256813418e-2_f64 * t93073 - 0.2032800112371413129e-3_f64 * t93077 + 0.28582678745379824648e-4_f64 * t93080 - 0.57165357490759649296e-4_f64 * t93084 - 0.80031500487063509015e-1_f64 * t93086 - 0.6097638132347695925e-3_f64 * t93088 + 0.28582678745379824648e-4_f64 * t93091 - t103347 + 0.10164000561857065645e-2_f64 * t93095;
    t103349
}
