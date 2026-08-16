//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 680/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk680<F: Float>(t1018: F, t1036: F, t138: F, t1040: F, t109: F, t1044: F, t1003: F, t1009: F, t1054: F, t1055: F, t1061: F, t1180: F, t269: F, t282: F, t30: F, t3719: F, t3834: F, t3842: F, t3851: F, t3859: F, t3862: F, t3867: F, t3871: F, t3874: F, t3877: F, t3881: F, t666: F, t668: F, t991: F, t992: F, t994: F) -> (F, F, F, F) {
    let t3884 = F::cast_from(0.053425_f64) * t138 * t1018 * t1036;
    let t3885 = t109 * t1040;
    let t3888 = F::cast_from(0.8591797547176487_f64) * t138 * t3885 * t1044;
    let t3889 = F::cast_from(0.03253074390090522_f64) * t138 * t3834 * t1055 + F::cast_from(0.10274_f64) * t138 * t109 * t991 * t994 - t3719 + F::cast_from(3.5089341735807875_f64) * t1061 * t3842 - F::cast_from(6.0_f64) * t992 * t668 * t1003 + F::cast_from(0.0016562821945185185_f64) * t30 * t1180 * t269 + F::cast_from(96.49187699215521_f64) * t1009 * t3851 * t666 + F::cast_from(0.0005696894717424259_f64) * t30 * t1180 * t282 + F::cast_from(51.94757731704439_f64) * t1061 * t3859 - F::cast_from(3.5089341735807875_f64) * t1054 * t3862 + t3867 - t3871 - t3874 - t3877 - t3881 + t3884 + t3888;
    (t3884, t3885, t3888, t3889)
}
