//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1081/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1081<F: Float>(t11529: F, t11534: F, t11536: F, t2841: F, t2843: F, t2845: F, t2894: F, t4039: F, t4048: F, t4052: F, t5508: F, t6579: F) -> F {
    let t19364 = t11529 + F::new(6.0) * t5508 + F::new(2.0) * t6579 + F::new(16.0) * t4039 - F::new(48.0) * t2841 - F::new(8.0) * t2843 - F::new(8.0) * t2845 + t11534 + t11536 - F::new(0.14649157844805236044e-2) * t4048 - F::new(48.0) * t2894 + F::new(12.0) * t4052;
    t19364
}
