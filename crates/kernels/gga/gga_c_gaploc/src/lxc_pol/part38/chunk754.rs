//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 754/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk754<F: Float>(t44817: F, t13552: F, t2549: F, t11595: F, t2508: F, t7667: F, t35682: F, t7659: F, t731: F, t13503: F, t13528: F, t13560: F, t1897: F, t2936: F, t33300: F, t33304: F, t33676: F, t44798: F, t44802: F, t44805: F, t44809: F, t44812: F, t650: F, t681: F, t9014: F) -> (F,) {
    let t44818 = 0.32043859292259267849e-3 * t44817;
    let t44819 = t2549 * t13552;
    let t44820 = 0.32043859292259267849e-3 * t44819;
    let t44823 = 0.53833683610995569986e-1 * t2508 * t11595 * t7667;
    let t44826 = 0.92286314761706691403e-1 * t2508 * t35682 * t7659;
    let t44827 = t731 * t13552;
    let t44828 = 0.42725145723012357132e-3 * t44827;
    let t44829 = t731 * t13503;
    let t44837 = t2549 * t13503;
    let t44845 = -t44798 + t44802 - t44805 - t44809 - t44812 + 0.46143157380853345702e-1 * t1897 * t2936 * t33676 + t44818 - t44820 - t44823 + t44826 + t44828 - 0.85450291446024714261e-3 * t44829 - 0.10766736722199113997e0 * t2508 * t2936 * t33300 + 0.18457262952341338281e0 * t2508 * t9014 * t33304 + 0.64087718584518535696e-3 * t44837 + 0.10254034973522965712e-1 * t650 * t13528 - 0.76905262301422242837e-2 * t681 * t13560 + 0.76905262301422242837e-2 * t681 * t13528;
    (t44845,)
}
