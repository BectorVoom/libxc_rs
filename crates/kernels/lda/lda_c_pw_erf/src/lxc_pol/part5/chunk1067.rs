//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1067/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1067<F: Float>(t50: F, t17673: F, t1789: F, t20019: F, t20027: F, t352: F, t4367: F, t4370: F, t52: F, t5997: F, t6005: F, t7365: F, t7370: F, t8334: F, t943: F, t950: F, zeta_threshold: F) -> F {
    let t51 = t50 <= zeta_threshold;
    let t20031 = piecewise3::<F>(t51, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t8334 * t7365 * t352 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t5997 * t943 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4367 * t17673 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t4370 * t20019 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1789 * t6005 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t950 * t7370 * t352 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t52 * t20027);
    t20031
}
