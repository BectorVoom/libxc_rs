//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1145/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1145<F: Float>(t24: F, t2569: F, t500: F, t1165: F, t19660: F, t19663: F, t19669: F, t19672: F, t3019: F, t3022: F, t333: F, t507: F, t5107: F, t5113: F, t7932: F, t7935: F, t7940: F, t82: F, zeta_threshold: F) -> F {
    let t90 = t24 <= zeta_threshold;
    let t19863 = F::cast_from(16.0_f64) * t2569 * t500;
    let t19865 = piecewise3::<F>(t90, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t7932 * t5107 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t7935 * t19660 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t3019 * t19663 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t7940 * t507 - F::cast_from(4.0_f64) * t3022 * t19669 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t3022 * t19672 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1165 * t5113 + F::cast_from(8.0_f64) * t333 * t82 - t19863);
    t19865
}
