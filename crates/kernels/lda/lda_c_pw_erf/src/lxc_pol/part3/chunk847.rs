//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 847/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk847<F: Float>(t1125: F, t153: F, t865: F, t1210: F, t168: F, t861: F, t1891: F, t474: F, t156: F, t3373: F, t4092: F, t4095: F, t4096: F, t4099: F, t4101: F, t4103: F, t4106: F, t4110: F, t4113: F, t5718: F) -> F {
    let t5904 = t153 * t1125 * t865;
    let t5907 = t168 * t1210 * t861;
    let t5911 = F::cast_from(1.1389037339096726_f64) * t153 * t474 * t1891;
    let t5920 = -t3373 + F::cast_from(1.328721022894618_f64) * t5904 - F::cast_from(0.053059442957798957_f64) * t5907 - t5911 - F::cast_from(0.1675256410710088_f64) * t4092 - t4095 - F::cast_from(0.3350512821420176_f64) * t4096 - t4099 + F::cast_from(0.0837628205355044_f64) * t4101 + F::cast_from(0.3350512821420176_f64) * t4103 + t4106 - F::cast_from(0.0837628205355044_f64) * t4110 + t4113 + F::cast_from(0.42708890021612717_f64) * t153 * t156 * t5718;
    t5920
}
