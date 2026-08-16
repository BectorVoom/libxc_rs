//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1070/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1070(t12396: f64, t12397: f64, t19618: f64, t350: f64, t7478: f64, t7498: f64, t19756: f64, t19760: f64, t19764: f64, t19768: f64, t19772: f64, t19774: f64, t19776: f64, t19780: f64, t19784: f64, t19788: f64, t19793: f64, t19796: f64, t19799: f64, t19804: f64, t19807: f64, t19811: f64, t9215: f64) -> (f64, f64, f64, f64) {
    let t19814 = t12396 * t12397 * t19618;
    let t19816 = t350 * t7478;
    let t19818 = t350 * t7498;
    let t19820 = 0.8638_f64 * t19756 - 0.07198333333333333_f64 * t19760 - 0.14396666666666666_f64 * t19764 + 0.8638_f64 * t19768 - 1.2957_f64 * t19772 + 0.023994444444444443_f64 * t19774 - 0.07198333333333333_f64 * t19776 + 0.21595_f64 * t19780 + 0.4319_f64 * t19784 - 0.8638_f64 * t19788 + 0.47988888888888886_f64 * t19793 - 0.10664197530864197_f64 * t19796 - 0.23994444444444443_f64 * t19799 + 0.07198333333333333_f64 * t19804 - 0.023994444444444443_f64 * t19807 + 0.03732469135802469_f64 * t9215 + 0.4319_f64 * t19811 - 0.11997222222222222_f64 * t19814 + 0.013330246913580247_f64 * t19816 + 0.011997222222222222_f64 * t19818;
    (t19814, t19816, t19818, t19820)
}
