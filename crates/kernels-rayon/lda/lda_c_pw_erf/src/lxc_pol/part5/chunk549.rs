//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 549/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk549(t169: f64, t242: f64, t2877: f64, t465: f64, t717: f64, t1098: f64, t632: f64, t1102: f64, t1143: f64, t699: f64, t703: f64, t161: f64, t2872: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2880 = 0.5188034422540342_f64 * t169 * t2877 * t242;
    let t2881 = t717 * t465;
    let t2883 = t169 * t2881 * t242;
    let t2887 = 0.42447554366239165_f64 * t169 * t1098 * t632;
    let t2893 = t169 * t1102 * t632;
    let t2897 = 0.15917832887339686_f64 * t169 * t699 * t1143;
    let t2906 = t169 * t703 * t1143;
    let t2908 = t2872 * t161;
    (t2880, t2881, t2883, t2887, t2893, t2897, t2906, t2908)
}
