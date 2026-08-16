//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1184/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1184(t231: f64, t25317: f64, t25383: f64, t26515: f64, t26551: f64, t2771: f64, t7070: f64, t7076: f64, t7398: f64, t92917: f64, t95615: f64, t95779: f64, t95783: f64, t95786: f64, t95790: f64, t95794: f64, t95796: f64, t95798: f64, t95807: f64, t95808: f64, t95811: f64, t95813: f64) -> f64 {
    let t95821 = 0.39029762157531132076e-1_f64 * t95779 - 0.72280234901709995519e-3_f64 * t95783 - 0.51405703062096148812e-1_f64 * t95786 + 0.38554277296572111609e-1_f64 * t95790 + 0.51405703062096148814e-2_f64 * t95794 + 0.28912093960683998208e-1_f64 * t95796 - 0.21684070470512998656e-1_f64 * t95798 + 0.13010442282307799193e1_f64 * t7070 * t7076 * t95615 * t231 + 0.13010442282307799193e1_f64 * t25383 * t26515 + t95807 - 0.68549505033305214441e-2_f64 * t95808 + 0.72280234901709995519e-3_f64 * t95811 - 0.68549505033305214441e-2_f64 * t95813 - 0.52041769129231196772e1_f64 * t92917 * t26551 - 0.78062653693846795158e1_f64 * t7070 * t25317 * t7398 * t2771;
    t95821
}
