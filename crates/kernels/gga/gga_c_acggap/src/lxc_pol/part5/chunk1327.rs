//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1327/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1327<F: Float>(t14712: F, t14717: F, t14718: F, t14719: F, t14720: F, t2959: F, t2961: F, t2963: F, t2966: F, t5397: F, t5401: F, t6590: F) -> F {
    let t24672 = -t14712 - F::cast_from(0.11696447245269292414e1_f64) * t2959 - F::cast_from(0.10389515463408878255e3_f64) * t2961 + F::cast_from(12.0_f64) * t5397 + F::cast_from(0.14649157844805236043e-2_f64) * t2963 - F::cast_from(0.36622894612013090108e-3_f64) * t2966 + t14717 - t14718 - t14719 + t14720 + F::cast_from(24.0_f64) * t6590 - F::cast_from(4.0_f64) * t5401;
    t24672
}
