//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 934/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk934<F: Float>(t1124: F, t265: F, t266: F, t3990: F, t640: F, t653: F, t1125: F, t252: F, t254: F, t1410: F, t1433: F, t1426: F, t635: F, t645: F) -> (F, F, F, F, F, F) {
    let t11097 = F::cast_from(56.0_f64) / F::cast_from(1215.0_f64) * t265 * t266 * t1124;
    let t11098 = t640 * t3990;
    let t11101 = F::cast_from(32.0_f64) / F::cast_from(81.0_f64) * t653 * t3990;
    let t11104 = F::cast_from(56.0_f64) / F::cast_from(243.0_f64) * t252 * t254 * t1125;
    let t11153 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1433 * t1410;
    let t11156 = F::cast_from(0.05402469135802469_f64) * t645 * t635 * t1426;
    (t11097, t11098, t11101, t11104, t11153, t11156)
}
