//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2121/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2121(t1955: f64, t27212: f64, t5977: f64, t7048: f64, t18784: f64, t1949: f64, t231: f64, t25383: f64, t27199: f64, t27300: f64, t27353: f64, t27358: f64, t29655: f64, t29691: f64, t62589: f64, t7070: f64, t7071: f64, t7076: f64, t93175: f64, t93177: f64, t99174: f64, t99212: f64, t99216: f64, t99219: f64, t99222: f64, t99228: f64, t99231: f64) -> (f64, f64, f64) {
    let t106172 = t1955 * t27212;
    let t106178 = t7048 * t5977;
    let t106190 = 0.8673628188205199462e0_f64 * t7070 * t7071 * t1949 * t18784 - 0.68540937416128198419e-2_f64 * t99212 + 0.8673628188205199462e0_f64 * t25383 * t29655 + t99216 + t99219 - 0.17347256376410398924e1_f64 * t106172 * t27358 - t99222 - 0.17135234354032049604e-2_f64 * t93175 + 0.4336814094102599731e0_f64 * t25383 * t29691 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t106178 * t231 - 0.52041769129231196772e1_f64 * t27199 * t27300 + 0.26020884564615598386e1_f64 * t27353 * t99174 * t62589 - 0.22849835011101738147e-2_f64 * t93177 + 0.39029762157531132076e-1_f64 * t99228 + t99231;
    (t106172, t106178, t106190)
}
