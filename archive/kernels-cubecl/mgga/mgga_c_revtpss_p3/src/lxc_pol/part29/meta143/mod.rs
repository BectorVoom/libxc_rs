//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta143 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk742;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk743;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta143<F: Float>(t1065: F, t999: F, t906: F, t1042: F, t2868: F, t2871: F, t2878: F, t2921: F, t2929: F, t3019: F, t3021: F, t3024: F, t3028: F, t3032: F, t3036: F, t1045: F, t373: F, t1031: F, t196: F) -> (F, F, F, F, F, F) {
        let (t3129, t3130, t3133) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk742::<F>(t1065, t999, t906, t1042, t2868, t2871, t2878, t2921, t2929, t3019, t3021, t3024, t3028, t3032, t3036);
        let (t3135, t3136, t3140) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk743::<F>(t1045, t3133, t373, t1042, t1031, t196);
    (t3129, t3130, t3133, t3135, t3136, t3140)
}
