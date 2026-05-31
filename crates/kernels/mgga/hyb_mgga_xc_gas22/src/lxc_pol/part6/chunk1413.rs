//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1413/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1413<F: Float>(t1100: F, t11217: F, t462: F, t2813: F, t4475: F, t22173: F, t22175: F, t22179: F, t22185: F, t22186: F, t22189: F, t22191: F, t22193: F, t22199: F, t22204: F, t22208: F, t22210: F, t22212: F, t22215: F, t22400: F, t30503: F, t495: F) -> F {
    let t30538 = t462 * t11217 * t1100;
    let t30547 = t462 * t4475 * t2813;
    let t30548 = -F::cast_from(8.0_f64) * t22173 + F::cast_from(64.0_f64) * t22175 - t22179 + t22185 + F::cast_from(0.11696447245269292414e1_f64) * t22186 + F::cast_from(0.20779030926817756511e3_f64) * t22189 - F::cast_from(24.0_f64) * t22191 + F::cast_from(120.0_f64) * t22193 + F::cast_from(2.0_f64) * t30538 + t22199 - F::cast_from(480.0_f64) * t22204 + F::cast_from(20.0_f64) * t22208 + F::cast_from(12.0_f64) * t22210 + F::cast_from(32.0_f64) * t22212 + t462 * t30503 * t495 + t30547 - t22215 + t22400;
    t30548
}
