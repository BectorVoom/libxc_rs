//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1127/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1127<F: Float>(t3905: F, t4763: F, t1446: F, t4887: F, t3794: F, t4882: F, t1325: F, t1440: F, t494: F, t5127: F, t529: F, t13162: F, t13164: F, t13166: F, t13171: F, t13175: F, t13177: F, t13179: F, t13182: F, t13187: F) -> (F, F, F, F, F) {
    let t13189 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t4763 * t3905;
    let t13191 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t1446 * t4887;
    let t13193 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t3794 * t4882;
    let t13198 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1325 * t1440 * t529 * t5127 * t494;
    let t13199 = -t13162 + t13164 - t13166 + t13171 + t13175 - t13177 + t13179 + t13182 - t13187 - t13189 + t13191 - t13193 - t13198;
    (t13189, t13191, t13193, t13198, t13199)
}
