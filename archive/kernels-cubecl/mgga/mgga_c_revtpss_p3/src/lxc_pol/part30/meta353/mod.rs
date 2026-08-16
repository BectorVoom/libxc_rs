//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1373;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta353<F: Float>(t3259: F, t359: F, t11239: F, t3143: F, t342: F, t1086: F, t3043: F, t3298: F, t989: F, t4980: F, t994: F, t4995: F) -> (F, F, F, F, F, F, F) {
        let (t12073, t12077, t12078, t12097, t12116, t12122, t12127) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1373::<F>(t3259, t359, t11239, t3143, t342, t1086, t3043, t3298, t989, t4980, t994, t4995);
    (t12073, t12077, t12078, t12097, t12116, t12122, t12127)
}
