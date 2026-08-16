//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1224/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1224(t10638: f64, t1949: f64, t1959: f64, t231: f64, t25317: f64, t25319: f64, t25349: f64, t25383: f64, t25392: f64, t2645: f64, t27353: f64, t2771: f64, t39620: f64, t7048: f64, t7070: f64, t7076: f64, t93206: f64, t93207: f64, t93210: f64, t93224: f64, t93226: f64, t93228: f64, t93231: f64, t93242: f64, t93244: f64) -> f64 {
    let t93250 = t93206 - 0.39029762157531132076e-1_f64 * t93207 - t93210 + 0.13010442282307799193e1_f64 * t7070 * t7076 * t7048 * t2645 * t231 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t1949 * t10638 * t231 + 0.13010442282307799193e1_f64 * t25383 * t25349 + t93224 - 0.21684070470512998656e-1_f64 * t93226 + 0.38554277296572111609e-1_f64 * t93228 - t93231 - 0.78062653693846795158e1_f64 * t25383 * t25319 - 0.78062653693846795158e1_f64 * t7070 * t25317 * t7048 * t2771 + 0.72280234901709995519e-3_f64 * t93242 - 0.4336814094102599731e0_f64 * t93244 * t1959 + 0.13010442282307799193e1_f64 * t27353 * t25392 * t39620;
    t93250
}
