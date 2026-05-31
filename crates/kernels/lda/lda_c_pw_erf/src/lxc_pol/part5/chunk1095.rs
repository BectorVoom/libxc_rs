//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1095/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1095<F: Float>(t14655: F, t10: F, t14796: F, t1832: F, t1856: F, t20359: F, t20371: F, t20374: F, t20376: F, t2610: F, t2624: F, t411: F, t426: F, t6121: F, t767: F, t7927: F, t7930: F) -> (F, F) {
    let t20390 = F::cast_from(5.84605_f64) * t14655;
    let t20391 = F::cast_from(0.73452_f64) * t20359 + F::cast_from(30.0_f64) * t426 * t10 * t7930 * t411 - F::cast_from(18.0_f64) * t426 * t10 * t2624 * t1832 - F::cast_from(8.81424_f64) * t20371 - t20374 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t20376 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t426 * t10 * t1856 * t2610 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t426 * t10 * t767 * t6121 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t426 * t10 * t7927 * t411 + t20390 - t14796;
    (t20390, t20391)
}
