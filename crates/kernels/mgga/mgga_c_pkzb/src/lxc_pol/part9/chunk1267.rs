//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1267/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1267<F: Float>(t24: F, t3022: F, t500: F, t1003: F, t1429: F, t18408: F, t19660: F, t19663: F, t19669: F, t19672: F, t2179: F, t3019: F, t507: F, t5107: F, t5113: F, t6097: F, t7932: F, t7935: F, t8: F, t82: F, t821: F, zeta_threshold: F) -> F {
    let t90 = t24 <= zeta_threshold;
    let t22258 = F::new(8.0) * t3022 * t500;
    let t22260 = piecewise3::<F>(t90, F::new(0.0), F::new(280.0) / F::new(81.0) * t18408 * t1003 * t5107 + F::new(56.0) / F::new(9.0) * t6097 * t8 * t19660 - F::new(28.0) / F::new(9.0) * t7932 * t19663 - F::new(8.0) / F::new(3.0) * t2179 * t1429 * t507 + F::new(8.0) * t7935 * t19669 - F::new(8.0) / F::new(3.0) * t7935 * t19672 + F::new(4.0) / F::new(9.0) * t3019 * t5113 - F::new(4.0) * t821 * t82 + t22258);
    t22260
}
