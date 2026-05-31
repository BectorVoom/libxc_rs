//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1259/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1259<F: Float>(t22593: F, t2337: F, t833: F, t352: F, t4506: F, t4522: F, t20823: F, t3974: F, t5160: F, t3976: F, t549: F, t593: F, t6728: F) -> (F, F, F, F, F, F) {
    let t22594 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t22593;
    let t22595 = t2337 * t833;
    let t22596 = t22595 * t352;
    let t22599 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t4506 * t4522 * t22596;
    let t22602 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t3974 * t5160 * t20823;
    let t22606 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t3974 * t3976 * t22595 * t549;
    let t22610 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4506 * t6728 * t22595 * t593;
    (t22594, t22596, t22599, t22602, t22606, t22610)
}
