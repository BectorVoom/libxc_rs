//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1328/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1328<F: Float>(t14725: F, t14726: F, t2971: F, t2984: F, t5026: F, t5032: F, t5036: F, t5405: F, t5409: F, t6020: F, t6594: F, t6598: F, t6601: F) -> F {
    let t24683 = F::cast_from(12.0_f64) * t5405 + F::cast_from(24.0_f64) * t5409 + F::cast_from(0.70178683471615754484e1_f64) * t5026 + F::cast_from(6.0_f64) * t6594 + F::cast_from(192.0_f64) * t2971 - t14725 - t14726 - F::cast_from(0.35089341735807877242e1_f64) * t2984 + F::cast_from(120.0_f64) * t5032 - t6020 - F::cast_from(64.0_f64) * t5036 + F::cast_from(4.0_f64) * t6598 + F::cast_from(24.0_f64) * t6601;
    t24683
}
