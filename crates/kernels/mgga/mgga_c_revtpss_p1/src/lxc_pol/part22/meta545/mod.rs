//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta545 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2358;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2359;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2360;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta545<F: Float>(t17649: F, t17650: F, t17350: F, t3767: F, t1121: F, t1248: F, t606: F, t3604: F, t17353: F, t372: F, t5277: F, t3630: F, t12784: F, t12866: F, t12910: F, t17619: F, t17622: F, t17625: F, t17629: F, t17635: F, t17640: F, t17646: F, t3625: F, t5402: F) -> (F, F, F, F, F, F, F, F) {
        let (t17651, t17654) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2358::<F>(t17649, t17650, t17350, t3767);
        let (t17656, t17657, t17658, t17661) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2359::<F>(t1121, t1248, t606, t3604, t17353, t372, t5277);
        let (t17662, t17665) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2360::<F>(t17661, t3630, t12784, t12866, t12910, t17619, t17622, t17625, t17629, t17635, t17640, t17646, t17651, t17654, t17658, t3625, t5402);
    (t17651, t17654, t17656, t17657, t17658, t17661, t17662, t17665)
}
