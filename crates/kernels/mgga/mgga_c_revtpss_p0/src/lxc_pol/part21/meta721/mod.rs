//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta721 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2560;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2561;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta721<F: Float>(t1331: F, t9855: F, t2619: F, t9563: F, t3825: F, t9586: F, t1333: F, t9342: F, t521: F, t583: F, t596: F, t525: F, t9603: F, t527: F, t9615: F, t1340: F, t40165: F, t2626: F, t9551: F, t512: F, t749: F, t9363: F, t268: F, t520: F, t39768: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47007, t47009, t47011, t47013, t47019, t47025) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2560::<F>(t1331, t9855, t2619, t9563, t3825, t9586, t1333, t9342, t521, t583, t596, t525, t9603);
        let (t47040, t47059, t47060, t47063, t47065, t47067) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2561::<F>(t527, t9615, t1340, t40165, t2626, t9551, t512, t749, t9363, t268, t520, t39768);
    (t47007, t47009, t47011, t47013, t47019, t47025, t47040, t47059, t47060, t47063, t47065, t47067)
}
