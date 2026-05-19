//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1332/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1332<F: Float>(t5697: F, t5950: F, t5702: F, t3010: F, t2990: F, t3002: F, t3004: F, t3018: F, t4304: F, t4308: F, t7350: F, t7353: F, t8118: F, t8120: F, t8121: F, t8122: F, t8123: F, t8126: F, t8130: F) -> F {
    let t15311 = F::new(24.0) * t5697;
    let t15312 = F::cast_from(2.464579730404_f64) * t5950;
    let t15315 = F::cast_from(0.0010986933022051897_f64) * t5702;
    let t15316 = F::new(24.0) * t3010;
    let t15320 = t7350 - t8118 - F::cast_from(155.84180309438278_f64) * t2990 - t8120 + t8121 - t7353 + t15311 - t15312 + t8122 - t8123 - F::cast_from(0.0010986933022051897_f64) * t3002 + F::cast_from(0.0021973866044103793_f64) * t3004 - t8126 - t15315 + t15316 + t8130 + F::cast_from(6.16144932601_f64) * t4304 - F::cast_from(2.464579730404_f64) * t4308 + F::new(9.0) * t3018;
    t15320
}
