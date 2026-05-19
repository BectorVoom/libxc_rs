//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 267/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk267<F: Float>(t260: F, t271: F, t785: F, t813: F, t816: F, t821: F, t830: F, t836: F, t840: F, t849: F, t856: F, t858: F) -> F {
    let t861 = -t785 + t813 + t260 * (-F::new(0.310907e-1) * t816 * t271 + F::new(1.0) * t821 * t830 + t785 - t813 - F::cast_from(0.19751673498613801407e-1_f64) * t836 + F::cast_from(0.5848223622634646207e0_f64) * t840 * t849) + F::cast_from(0.19751673498613801407e-1_f64) * t260 * t836 - F::cast_from(0.5848223622634646207e0_f64) * t856 * t858;
    t861
}
