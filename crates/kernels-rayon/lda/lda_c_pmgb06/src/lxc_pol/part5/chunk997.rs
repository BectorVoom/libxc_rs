//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 997/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk997(t2489: f64, t3226: f64, t1447: f64, t6292: f64, t441: f64, t6673: f64, t224: f64, t6687: f64, t118: f64, t5988: f64, t2414: f64, t740: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18008 = t3226 * t2489;
    let t18010 = t1447 * t6292;
    let t18016 = t441 * t6673;
    let t18020 = t6687 * t224;
    let t18054 = t5988 * t118;
    let t18057 = t740 * t2414;
    (t18008, t18010, t18016, t18020, t18054, t18057)
}
