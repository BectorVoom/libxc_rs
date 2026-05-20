//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1480;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1481;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta270<F: Float>(t10199: F, t2851: F, t78: F, t3361: F, t81: F, t157: F, t36: F, t200: F, t45: F, t202: F, t57: F, t2435: F, t2445: F, t2441: F, t9303: F, t10115: F, t258: F, t2453: F, t2464: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10379, t10389, t10398, t10439, t10446, t10457, t10498) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1480::<F>(t10199, t2851, t78, t3361, t81, t157, t36, t200, t45, t202, t57, t2435, t2445);
        let (t10501, t10503, t10504) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1481::<F>(t2441, t9303, t10115, t258, t2453, t2464);
    (t10379, t10389, t10398, t10439, t10446, t10457, t10498, t10501, t10503, t10504)
}
