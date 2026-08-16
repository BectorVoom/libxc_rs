//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1357/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1357(t10675: f64, t10685: f64, t10688: f64, t10690: f64, t10694: f64, t10697: f64, t10702: f64, t10704: f64, t10709: f64, t14256: f64, t23067: f64, t23069: f64, t23070: f64) -> f64 {
    let t23340 = -t14256 - t23067 - t23069 - t23070 + t10675 + t10685 + 0.21642082724729686_f64 * t10688 - 0.09618703433213194_f64 * t10690 - t10694 + t10697 + 0.3246312408709453_f64 * t10702 + 0.03354522822333102_f64 * t10704 + t10709;
    t23340
}
