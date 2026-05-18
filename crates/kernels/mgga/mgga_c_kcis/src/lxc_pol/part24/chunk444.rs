//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 444/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk444<F: Float>(t260: F, t2917: F, t45: F, t956: F, t270: F, t961: F) -> (F, F, F, F, F) {
    let t2987 = t260 * t260;
    let t2988 = F::new(1.0) / t2987;
    let t2992 = F::new(0.12361111111111111111e-1) * t2917;
    let t3001 = t45 * t956;
    let t3004 = t961 * t270;
    let t3005 = F::new(1.0) / t3004;
    (t2987, t2988, t2992, t3001, t3005)
}
