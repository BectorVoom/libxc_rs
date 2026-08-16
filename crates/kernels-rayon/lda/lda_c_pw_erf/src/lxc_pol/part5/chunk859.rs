//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 859/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk859(t770: f64, t142: f64, t2776: f64, t2630: f64, t1554: f64, t100: f64, t7918: f64, t2675: f64, t6086: f64, t1809: f64, t2591: f64, t5799: f64, t5801: f64, t6162: f64, t7460: f64, t7462: f64, t7464: f64, t7468: f64, t7472: f64, t7473: f64, t7477: f64, t7481: f64, t7483: f64, t7487: f64, t7491: f64, t7493: f64, t7494: f64, t7495: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7986 = t770 * t770;
    let t7987 = t142 * t7986;
    let t7988 = t2776 * t7987;
    let t7991 = t142 * t2630;
    let t7992 = t1554 * t7991;
    let t7996 = t7918 * t100;
    let t8001 = t6086 * t2675;
    let t8004 = t2591 * t1809;
    let t8010 = t5799 + 0.36466666666666664_f64 * t5801 - t7460 - t7462 - t7464 + t7468 + t7472 - t7473 + t7477 + t7481 - t7483 - t7487 - t7491 - t7493 - 2.0_f64 / 15.0_f64 * t6162 + t7494 + t7495;
    (t7986, t7987, t7988, t7991, t7992, t7996, t8001, t8004, t8010)
}
