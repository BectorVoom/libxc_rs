//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1236/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1236<F: Float>(t3202: F, t3955: F, t14113: F, t14614: F, t14001: F, t14463: F, t3291: F, t51214: F, t14063: F, t8962: F, t51201: F, t51222: F) -> (F, F, F, F, F, F, F) {
    let t53970 = t3955 * t3202;
    let t53975 = t14113 * t14614;
    let t53985 = t14001 * t14463;
    let t54014 = t51214 * t3291;
    let t54023 = t14063 * t8962;
    let t54026 = F::new(119.0) / F::new(1728.0) * t51201;
    let t54038 = F::new(35.0) / F::new(216.0) * t51222;
    (t53970, t53975, t53985, t54014, t54023, t54026, t54038)
}
