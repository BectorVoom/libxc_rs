//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1158/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1158(t10743: f64, t10746: f64, t10757: f64, t10760: f64, t10764: f64, t10769: f64, t10770: f64, t10773: f64, t10777: f64, t17787: f64, t17790: f64, t20914: f64) -> f64 {
    let t20915 = 0.21642082724729686_f64 * t10743 + t10746 + 0.3246312408709453_f64 * t10757 + 0.03354522822333102_f64 * t10760 + t10764 + t10769 - 0.09618703433213194_f64 * t10770 - t10773 + t10777 + 4.0_f64 * t17787 + 4.0_f64 * t17790 - t20914;
    t20915
}
