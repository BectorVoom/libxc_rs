//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1270/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1270(t185: f64, t186: f64, t22664: f64, t22700: f64, t22745: f64, t22806: f64, t530: f64, t18404: f64, t18407: f64, t18409: f64, t18413: f64, t22616: f64, t22619: f64, t22622: f64, t22624: f64, t22626: f64, t22629: f64, t22631: f64, t22634: f64, t22636: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22812 = 2.0_f64 / 15.0_f64 * t185 * t186 * t530 * (t22664 + t22700 + t22745 + t22806);
    let t22813 = 16.0_f64 / 45.0_f64 * t18404;
    let t22814 = 16.0_f64 / 45.0_f64 * t18407;
    let t22815 = 32.0_f64 / 45.0_f64 * t18409;
    let t22816 = 16.0_f64 / 27.0_f64 * t18413;
    let t22817 = -t22616 - t22619 - t22622 - t22624 - t22626 - t22629 - t22631 - t22634 - t22636 - t22812 - t22813 - t22814 - t22815 + t22816;
    (t22812, t22813, t22814, t22815, t22816, t22817)
}
