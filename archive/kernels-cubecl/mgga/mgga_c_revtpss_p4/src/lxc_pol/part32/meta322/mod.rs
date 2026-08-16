//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta322 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1240;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta322<F: Float>(t11239: F, t3143: F, t342: F, t3298: F, t989: F, t4980: F, t994: F, t4995: F, t1043: F, t3153: F, t3046: F, t3286: F) -> (F, F, F, F, F, F, F) {
        let (t12077, t12078, t12116, t12122, t12127, t12131, t12146) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1240::<F>(t11239, t3143, t342, t3298, t989, t4980, t994, t4995, t1043, t3153, t3046, t3286);
    (t12077, t12078, t12116, t12122, t12127, t12131, t12146)
}
