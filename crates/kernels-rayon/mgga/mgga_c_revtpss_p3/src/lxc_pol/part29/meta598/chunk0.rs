//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2027/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2027(t103431: f64, t25375: f64, t212: f64, t28340: f64, t689: f64, t780: f64, t103182: f64, t93281: f64, t103421: f64, t7058: f64, t25317: f64, t25383: f64, t26475: f64, t28385: f64, t28405: f64, t28417: f64, t28436: f64, t7070: f64, t7415: f64, t7766: f64, t8012: f64, t886: f64, t92917: f64, t93126: f64, t95930: f64, t95937: f64, t95945: f64, t95948: f64, t99303: f64) -> f64 {
    let t103521 = t25375 * t103431;
    let t103529 = 0.10975748638225852664e-1_f64 * t689 * t212 * t28340 * t780;
    let t103543 = t93281 * t103182;
    let t103547 = t7058 * t103421;
    let t103549 = -t95930 + 0.19514881078765566038e-1_f64 * t95937 + 0.19274729307122665472e-1_f64 * t103521 + 0.34270468708064099208e-2_f64 * t95945 - 0.17347256376410398924e1_f64 * t92917 * t28436 - t103529 + 0.4336814094102599731e0_f64 * t93126 * t8012 + 0.8673628188205199462e0_f64 * t25383 * t28385 + 0.8673628188205199462e0_f64 * t25383 * t28405 - 0.4336814094102599731e0_f64 * t7766 * t26475 - 0.52041769129231196772e1_f64 * t7070 * t25317 * t28417 * t886 + 0.23131639038696784278e-2_f64 * t95948 + 0.86736281882051994623e-1_f64 * t103543 + 0.17347256376410398924e1_f64 * t99303 * t7415 - 0.96373646535613327357e-2_f64 * t103547;
    t103549
}
