//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1221/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1221<F: Float>(t1318: F, t1319: F, t1446: F, t15931: F, t2146: F, t2389: F, t2419: F, t34: F, t3416: F, t4753: F, t4758: F, t4763: F, t5334: F, t6275: F, t6397: F, t6479: F, t6483: F, t6665: F, t743: F, t7734: F, t7803: F, t7822: F, t811: F) -> F {
    let t22048 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4753 * t7734 + F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t3416 * t7734 + F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1318 * t1319 * t6665 * t811 - F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1318 * t4758 * t2419 * t34 - F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1446 * t7803 - F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t4763 * t6479 - F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t4763 * t6483 + F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4753 * t7822 + F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t3416 * t7822 + F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1318 * t1319 * t15931 * t743 - F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1318 * t4758 * t6275 * t34 - F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t5334 * t2389 - F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t2146 * t6397;
    t22048
}
