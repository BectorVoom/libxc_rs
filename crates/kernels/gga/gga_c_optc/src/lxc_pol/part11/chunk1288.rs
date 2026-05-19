//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1288/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1288<F: Float>(t39545: F, t39560: F, t49395: F, t57012: F, t57016: F, t57020: F, t57024: F, t57027: F, t57030: F, t57034: F, t57037: F, t57041: F, t57044: F, t57048: F) -> F {
    let t57052 = F::cast_from(0.44291358024691358024e0_f64) * t49395 + F::new(0.197176e1) * t57012 + F::cast_from(0.49293999999999999999e0_f64) * t57016 - F::cast_from(0.82156666666666666668e-1_f64) * t57020 - F::cast_from(0.71752000000000000002e1_f64) * t57024 - F::cast_from(0.59793333333333333333e0_f64) * t57027 - F::cast_from(0.98587999999999999998e0_f64) * t57030 + F::cast_from(0.43816888888888888889e0_f64) * t57034 + F::cast_from(0.39862222222222222223e1_f64) * t57037 - F::cast_from(0.88582716049382716048e0_f64) * t57041 - F::cast_from(0.85199506172839506175e-1_f64) * t57044 - F::cast_from(0.82156666666666666667e-1_f64) * t57048 - F::cast_from(0.18257037037037037037e0_f64) * t39545 - F::cast_from(0.54771111111111111111e0_f64) * t39560;
    t57052
}
