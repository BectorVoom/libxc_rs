//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 868/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk868(t13552: f64, t731: f64, t13503: f64, t2549: f64, t13528: f64, t13560: f64, t1897: f64, t2508: f64, t2936: f64, t33300: f64, t33304: f64, t33676: f64, t44798: f64, t44802: f64, t44805: f64, t44809: f64, t44812: f64, t44818: f64, t44820: f64, t44823: f64, t44826: f64, t650: f64, t681: f64, t9014: f64) -> f64 {
    let t44827 = t731 * t13552;
    let t44828 = 0.42725145723012357132e-3_f64 * t44827;
    let t44829 = t731 * t13503;
    let t44837 = t2549 * t13503;
    let t44845 = -t44798 + t44802 - t44805 - t44809 - t44812 + 0.46143157380853345702e-1_f64 * t1897 * t2936 * t33676 + t44818 - t44820 - t44823 + t44826 + t44828 - 0.85450291446024714261e-3_f64 * t44829 - 0.10766736722199113997e0_f64 * t2508 * t2936 * t33300 + 0.18457262952341338281e0_f64 * t2508 * t9014 * t33304 + 0.64087718584518535696e-3_f64 * t44837 + 0.10254034973522965712e-1_f64 * t650 * t13528 - 0.76905262301422242837e-2_f64 * t681 * t13560 + 0.76905262301422242837e-2_f64 * t681 * t13528;
    t44845
}
