//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1087/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1087<F: Float>(t43: F, t12161: F, t1281: F, t15072: F, t1690: F, t1694: F, t19461: F, t234: F, t2868: F, t2898: F, t35: F, t4070: F, t5455: F, t5481: F, t5486: F, t595: F, t817: F, t818: F, t821: F, t824: F, zeta_threshold: F) -> F {
    let t44 = t43 <= zeta_threshold;
    let t19482 = piecewise3::<F>(t44, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t12161 * t1690 * t818 + F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t4070 * t19461 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t5481 * t824 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t817 * t35 * t595 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1281 * t821 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1281 * t2868 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t2898 * t1694 * t818 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t817 * t5455 * t234 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5486 * t824 + t15072);
    t19482
}
