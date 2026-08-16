//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 544/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk544(t118: f64, t2813: f64, t1329: f64, t415: f64, t1186: f64, t1334: f64, t421: f64, t2777: f64, t2780: f64, t2793: f64, t2794: f64, t2797: f64, t2804: f64, t2807: f64, t2809: f64, t2812: f64) -> (f64, f64, f64, f64) {
    let t2814 = t2813 * t118;
    let t2816 = t1329 * t415;
    let t2820 = 0.01975389032890948_f64 * t1334 * t1186 * t421;
    let t2821 = t2777 - t2780 - t2793 - 0.09451622166942335_f64 * t2794 + t2797 - 0.031505407223141116_f64 * t2804 * t118 - 0.09451622166942335_f64 * t2807 - 0.1890324433388467_f64 * t2809 - t2812 + 0.09451622166942335_f64 * t2814 + 0.1890324433388467_f64 * t2816 + t2820;
    (t2814, t2816, t2820, t2821)
}
