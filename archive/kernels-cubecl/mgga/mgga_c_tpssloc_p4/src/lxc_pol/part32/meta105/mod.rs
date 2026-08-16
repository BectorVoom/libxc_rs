//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta105 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk659;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk660;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta105<F: Float>(t138: F, t681: F, t125: F, t2412: F, t702: F, t118: F, t142: F, t2393: F) -> (F, F, F, F, F) {
        let (t2419, t2420, t2421, t2423) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk659::<F>(t138, t681, t125, t2412, t702);
        let t2426 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk660::<F>(t118, t142, t2393);
    (t2419, t2420, t2421, t2423, t2426)
}
