//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2027/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2027<F: Float>(t103431: F, t25375: F, t212: F, t28340: F, t689: F, t780: F, t103182: F, t93281: F, t103421: F, t7058: F, t25317: F, t25383: F, t26475: F, t28385: F, t28405: F, t28417: F, t28436: F, t7070: F, t7415: F, t7766: F, t8012: F, t886: F, t92917: F, t93126: F, t95930: F, t95937: F, t95945: F, t95948: F, t99303: F) -> F {
    let t103521 = t25375 * t103431;
    let t103529 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t212 * t28340 * t780;
    let t103543 = t93281 * t103182;
    let t103547 = t7058 * t103421;
    let t103549 = -t95930 + F::cast_from(0.19514881078765566038e-1_f64) * t95937 + F::cast_from(0.19274729307122665472e-1_f64) * t103521 + F::cast_from(0.34270468708064099208e-2_f64) * t95945 - F::cast_from(0.17347256376410398924e1_f64) * t92917 * t28436 - t103529 + F::cast_from(0.4336814094102599731e0_f64) * t93126 * t8012 + F::cast_from(0.8673628188205199462e0_f64) * t25383 * t28385 + F::cast_from(0.8673628188205199462e0_f64) * t25383 * t28405 - F::cast_from(0.4336814094102599731e0_f64) * t7766 * t26475 - F::cast_from(0.52041769129231196772e1_f64) * t7070 * t25317 * t28417 * t886 + F::cast_from(0.23131639038696784278e-2_f64) * t95948 + F::cast_from(0.86736281882051994623e-1_f64) * t103543 + F::cast_from(0.17347256376410398924e1_f64) * t99303 * t7415 - F::cast_from(0.96373646535613327357e-2_f64) * t103547;
    t103549
}
