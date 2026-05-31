//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1287/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1287<F: Float>(t4738: F, t6946: F, t2171: F, t6905: F, t6909: F, t2146: F, t6685: F, t18485: F, t18487: F, t18490: F, t18492: F, t18505: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23035 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t4738 * t6946;
    let t23037 = F::cast_from(12.0_f64) / F::cast_from(5.0_f64) * t2171 * t6905;
    let t23039 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t2171 * t6909;
    let t23040 = t2146 * t6685;
    let t23041 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t23040;
    let t23042 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t18485;
    let t23043 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t18487;
    let t23044 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t18490;
    let t23045 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t18492;
    let t23046 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t18505;
    (t23035, t23037, t23039, t23041, t23042, t23043, t23044, t23045, t23046)
}
