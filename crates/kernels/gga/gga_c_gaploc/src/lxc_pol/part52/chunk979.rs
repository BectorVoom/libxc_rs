//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 979/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk979<F: Float>(t44084: F, t44088: F, t45869: F, t45874: F, t45877: F, t45882: F, t45885: F, t45887: F, t45888: F, t45892: F, t45894: F, t45898: F, t45900: F, t45903: F, t45906: F, t47488: F, t47492: F, t47494: F, t47506: F, t47509: F) -> F {
    let t50286 = t45869 - t45874 + t45877 - F::new(0.63904876589867916127e-1) * t44084 - F::new(0.63904876589867916127e-1) * t44088 - F::new(0.59584149919750711116e-1) * t47488 - F::new(0.59584149919750711116e-1) * t47492 + F::new(0.76685851907841499354e0) * t47494 + F::new(0.17041300423964777634e0) * t47506 + t45882 + t45885 + F::new(0.59584149919750711116e-1) * t47509 + t45887 + F::new(0.89376224879626066674e-1) * t45888 - t45892 - t45894 - t45898 - t45900 + t45903 - t45906;
    t50286
}
