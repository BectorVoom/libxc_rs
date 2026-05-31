//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 943/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk943<F: Float>(t3124: F, t7884: F, t2024: F, t2027: F, t3288: F, t6471: F, t6474: F, t6477: F, t6481: F, t677: F, t684: F, t687: F, t8560: F, t8562: F, t8566: F, t8570: F, t8575: F, t8577: F) -> (F, F) {
    let t8579 = t7884 * t3124;
    let t8583 = t6471 / F::cast_from(144.0_f64) - t6474 / F::cast_from(96.0_f64) - t6477 / F::cast_from(192.0_f64) - t6481 / F::cast_from(144.0_f64) - t8560 - t684 * t687 * t8562 / F::cast_from(32.0_f64) - t684 * t687 * t8566 / F::cast_from(64.0_f64) - t2024 * t2027 * t8570 / F::cast_from(48.0_f64) + t8575 / F::cast_from(288.0_f64) + t8577 / F::cast_from(96.0_f64) - F::cast_from(7.0_f64) / F::cast_from(32.0_f64) * t8579 - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t677 * t3288;
    (t8579, t8583)
}
