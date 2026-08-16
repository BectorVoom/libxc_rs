//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 539/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk539(t174: f64, t2749: f64, t335: f64, t936: f64, t998: f64, t155: f64, t912: f64, t914: f64, t1035: f64, t344: f64, t137: f64, t142: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2751 = t174 * t2749 * t335;
    let t2752 = 0.07123333333333333_f64 * t2751;
    let t2754 = t174 * t998 * t936;
    let t2755 = 0.053425_f64 * t2754;
    let t2758 = t174 * t155 * t912 * t914;
    let t2759 = 0.10685_f64 * t2758;
    let t2760 = t344 * t1035;
    let t2761 = 12.0_f64 * t2760;
    let t2765 = t137 * t142;
    (t2751, t2752, t2754, t2755, t2758, t2759, t2760, t2761, t2765)
}
