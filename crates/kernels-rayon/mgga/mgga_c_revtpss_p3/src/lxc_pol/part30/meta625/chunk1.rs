//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2163/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2163(t27303: f64, t786: f64, t789: f64, t25296: f64, t27216: f64, t25407: f64, t27353: f64, t27357: f64, t51529: f64, t7775: f64, t7779: f64, t93126: f64, t93158: f64, t93161: f64, t93167: f64, t93175: f64, t93177: f64, t99202: f64, t99206: f64, t99212: f64, t99216: f64) -> f64 {
    let t99219 = 0.19514881078765566038e-1_f64 * t786 * t27303 * t789;
    let t99222 = 0.25702851531048074406e-1_f64 * t27216 * t25296;
    let t99227 = -0.17347256376410398924e1_f64 * t27353 * t27357 * t51529 + 0.34270468708064099208e-2_f64 * t93158 - 0.45699670022203476294e-2_f64 * t93161 + 0.17135234354032049604e-1_f64 * t99202 - 0.24093411633903331839e-3_f64 * t99206 + 0.4336814094102599731e0_f64 * t93126 * t7775 - 0.3427046870806409921e-2_f64 * t99212 + t99216 + t99219 - 0.12851425765524037203e-1_f64 * t93167 - t99222 - 0.34270468708064099208e-2_f64 * t93175 - 0.4336814094102599731e0_f64 * t25407 * t7779 - 0.45699670022203476294e-2_f64 * t93177;
    t99227
}
