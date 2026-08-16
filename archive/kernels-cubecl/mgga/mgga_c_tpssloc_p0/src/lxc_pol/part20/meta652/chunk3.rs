//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2403/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2403<F: Float>(t41959: F, t41962: F, t47781: F, t47785: F, t47787: F, t49043: F, t49049: F, t49052: F, t49054: F, t49056: F, t49058: F, t49060: F) -> F {
    let t49219 = F::cast_from(0.58258125e1_f64) * t49043 + t41959 + t41962 - F::cast_from(0.10064166666666666667e1_f64) * t47781 - F::cast_from(0.543465e1_f64) * t47785 + F::cast_from(0.31310740740740740741e0_f64) * t47787 - F::cast_from(0.3883875e1_f64) * t49049 + F::cast_from(0.247573125e0_f64) * t49052 + F::cast_from(0.247573125e0_f64) * t49054 + F::cast_from(0.82524375e-1_f64) * t49056 - F::cast_from(0.3883875e1_f64) * t49058 + F::cast_from(0.258925e1_f64) * t49060;
    t49219
}
