//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 901/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk901<F: Float>(t10085: F, t3876: F, t3881: F, t9787: F, t3877: F, t8392: F, t3882: F, t3888: F, t1882: F, t3979: F, t4005: F, t713: F, t729: F) -> (F, F, F, F, F, F, F) {
    let t13952 = t10085 * t3876;
    let t13955 = t9787 * t3881;
    let t13959 = F::new(2.0) / F::new(27.0) * t8392 * t3877;
    let t13961 = F::new(2.0) / F::new(27.0) * t8392 * t3882;
    let t13963 = F::new(4.0) / F::new(27.0) * t8392 * t3888;
    let t13965 = F::new(2.0) / F::new(9.0) * t1882 * t3979;
    let t13967 = t729 * t4005 * t713;
    (t13952, t13955, t13959, t13961, t13963, t13965, t13967)
}
