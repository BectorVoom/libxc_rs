//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 926/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk926(t285: f64, t2872: f64, t695: f64, t1063: f64, t274: f64, t169: f64, t242: f64, t2818: f64, t2929: f64, t703: f64, t699: f64, t1102: f64, t1143: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10872 = 0.0011622696607154768_f64 * t695 * t2872 * t285;
    let t10878 = 6.399008129061525_f64 * t1063 * t274;
    let t10881 = 2.4210827305188265_f64 * t169 * t2818 * t242;
    let t10886 = t169 * t703 * t2929;
    let t10897 = 0.21223777183119583_f64 * t169 * t699 * t2929;
    let t10906 = t169 * t1102 * t1143;
    (t10872, t10878, t10881, t10886, t10897, t10906)
}
