//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1328/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1328<F: Float>(t14725: F, t14726: F, t2971: F, t2984: F, t5026: F, t5032: F, t5036: F, t5405: F, t5409: F, t6020: F, t6594: F, t6598: F, t6601: F) -> F {
    let t24683 = F::new(12.0) * t5405 + F::new(24.0) * t5409 + F::cast_from(0.70178683471615754484e1_f64) * t5026 + F::new(6.0) * t6594 + F::new(192.0) * t2971 - t14725 - t14726 - F::cast_from(0.35089341735807877242e1_f64) * t2984 + F::new(120.0) * t5032 - t6020 - F::new(64.0) * t5036 + F::new(4.0) * t6598 + F::new(24.0) * t6601;
    t24683
}
