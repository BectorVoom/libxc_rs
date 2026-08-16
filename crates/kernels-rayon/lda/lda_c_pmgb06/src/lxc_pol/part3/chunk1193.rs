//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1193/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1193(t10679: f64, t10681: f64, t10684: f64, t11893: f64, t11894: f64, t11895: f64, t11898: f64, t11902: f64, t11906: f64, t11910: f64, t11912: f64, t11915: f64, t11918: f64, t11934: f64, t11937: f64, t11940: f64, t11943: f64, t11946: f64, t11951: f64, t11953: f64, t11955: f64, t11959: f64, t11970: f64) -> (f64, f64) {
    let t14330 = t11893 + t11894 + t11895 + t11898 + 0.21642082724729686_f64 * t10679 - 0.03354522822333102_f64 * t10681 - t10684 + t11902 + t11906 + t11910 - t11912;
    let t14331 = -t11915 - t11918 + t11934 - t11937 - t11940 - t11943 - t11946 - t11951 - t11953 + t11955 + t11959 - t11970;
    (t14330, t14331)
}
