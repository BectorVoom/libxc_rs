//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta560 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2116;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta560<F: Float>(t10660: F, t888: F, t10810: F, t919: F, t2859: F, t2884: F, t302: F, t41654: F, t41961: F, t2887: F, t10619: F, t300: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t42143, t42149, t42154, t42212, t42213, t42226, t42228, t42245, t42281) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2116::<F>(t10660, t888, t10810, t919, t2859, t2884, t302, t41654, t41961, t2887, t10619, t300);
    (t42143, t42149, t42154, t42212, t42213, t42226, t42228, t42245, t42281)
}
