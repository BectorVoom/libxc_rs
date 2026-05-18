//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1322/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1322<F: Float>(t495: F, t811: F, t11898: F, t11900: F, t1272: F, t1670: F, t1674: F, t1679: F, t1680: F, t20025: F, t20027: F, t20031: F, t20032: F, t20033: F, t20092: F, t3988: F, t4818: F, t5392: F, t694: F, t96: F) -> F {
    let t24623 = t495 * t811;
    let t24633 = F::new(12.0) * t1272 * t20092 * t96 + F::new(24.0) * t1670 * t1674 * t4818 - F::new(2.0) * t1679 * t1680 * t5392 + F::new(12.0) * t24623 * t3988 * t694 + t11898 + t11900 + t20025 + t20027 - t20031 + t20032 + t20033;
    t24633
}
