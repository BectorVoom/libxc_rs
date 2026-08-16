//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 30/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk30<F: Float>(t11: F, t14: F, t17: F, t25: F) -> (F, F, F) {
    let t67 = F::cast_from(0.705945e1_f64) * t14 + F::cast_from(0.1549425e1_f64) * t11 + F::cast_from(0.420775e0_f64) * t17 + F::cast_from(0.1562925e0_f64) * t25;
    let t70 = F::cast_from(1.0_f64) + F::cast_from(0.32163958997385070134e2_f64) / t67;
    let t71 = F::ln(t70);
    (t67, t70, t71)
}
