//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1598/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1598<F: Float>(t43858: F, t43862: F, t43865: F, t43871: F, t43877: F, t43883: F, t43909: F, t43911: F, t43914: F, t43917: F, t43920: F, t43923: F, t43926: F, t43928: F) -> F {
    let t44067 = F::new(0.1898925e1) * t43909 - F::cast_from(0.18257037037037037037e0_f64) * t43911 - F::cast_from(0.82156666666666666668e-1_f64) * t43914 + F::cast_from(0.49293999999999999999e0_f64) * t43917 - F::cast_from(0.10954222222222222222e0_f64) * t43920 - F::cast_from(0.98587999999999999999e0_f64) * t43923 + F::cast_from(0.65725333333333333332e0_f64) * t43926 + F::cast_from(0.21908444444444444444e0_f64) * t43928 - F::cast_from(0.44291358024691358024e0_f64) * t43858 - F::cast_from(0.88582716049382716048e0_f64) * t43862 - F::cast_from(0.5314962962962962963e0_f64) * t43865 - F::cast_from(0.59793333333333333333e0_f64) * t43871 + F::new(0.17938e1) * t43877 + F::cast_from(0.15944888888888888889e1_f64) * t43883;
    t44067
}
