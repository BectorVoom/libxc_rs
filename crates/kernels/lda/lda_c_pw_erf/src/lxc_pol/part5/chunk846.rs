//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 846/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk846<F: Float>(t6263: F, t739: F, t1326: F, t1325: F, t2171: F, t2397: F, t4041: F, t4215: F, t4217: F, t7736: F, t7740: F, t7744: F, t7748: F, t7751: F, t7754: F, t7755: F, t7757: F, t7796: F, t7801: F, t7805: F, t7807: F) -> (F, F, F, F, F) {
    let t7808 = t6263 * t739;
    let t7809 = t1326 * t7808;
    let t7811 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1325 * t7809;
    let t7813 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t2171 * t2397;
    let t7814 = t7736 - t7740 + t7744 + t7748 - t7751 - t7754 + t4041 + t7755 + t4215 + t4217 - t7757 - t7796 + t7801 - t7805 + t7807 + t7811 - t7813;
    (t7808, t7809, t7811, t7813, t7814)
}
