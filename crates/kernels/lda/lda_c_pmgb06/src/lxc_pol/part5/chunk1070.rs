//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1070/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1070<F: Float>(t12396: F, t12397: F, t19618: F, t350: F, t7478: F, t7498: F, t19756: F, t19760: F, t19764: F, t19768: F, t19772: F, t19774: F, t19776: F, t19780: F, t19784: F, t19788: F, t19793: F, t19796: F, t19799: F, t19804: F, t19807: F, t19811: F, t9215: F) -> (F, F, F, F) {
    let t19814 = t12396 * t12397 * t19618;
    let t19816 = t350 * t7478;
    let t19818 = t350 * t7498;
    let t19820 = F::new(0.8638) * t19756 - F::cast_from(0.07198333333333333_f64) * t19760 - F::cast_from(0.14396666666666666_f64) * t19764 + F::new(0.8638) * t19768 - F::new(1.2957) * t19772 + F::cast_from(0.023994444444444443_f64) * t19774 - F::cast_from(0.07198333333333333_f64) * t19776 + F::new(0.21595) * t19780 + F::new(0.4319) * t19784 - F::new(0.8638) * t19788 + F::cast_from(0.47988888888888886_f64) * t19793 - F::cast_from(0.10664197530864197_f64) * t19796 - F::cast_from(0.23994444444444443_f64) * t19799 + F::cast_from(0.07198333333333333_f64) * t19804 - F::cast_from(0.023994444444444443_f64) * t19807 + F::cast_from(0.03732469135802469_f64) * t9215 + F::new(0.4319) * t19811 - F::cast_from(0.11997222222222222_f64) * t19814 + F::cast_from(0.013330246913580247_f64) * t19816 + F::cast_from(0.011997222222222222_f64) * t19818;
    (t19814, t19816, t19818, t19820)
}
