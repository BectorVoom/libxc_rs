//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta721 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2560;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2561;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta721(t1331: f64, t9855: f64, t2619: f64, t9563: f64, t3825: f64, t9586: f64, t1333: f64, t9342: f64, t521: f64, t583: f64, t596: f64, t525: f64, t9603: f64, t527: f64, t9615: f64, t1340: f64, t40165: f64, t2626: f64, t9551: f64, t512: f64, t749: f64, t9363: f64, t268: f64, t520: f64, t39768: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47007, t47009, t47011, t47013, t47019, t47025) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2560(t1331, t9855, t2619, t9563, t3825, t9586, t1333, t9342, t521, t583, t596, t525, t9603);
        let (t47040, t47059, t47060, t47063, t47065, t47067) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2561(t527, t9615, t1340, t40165, t2626, t9551, t512, t749, t9363, t268, t520, t39768);
    (t47007, t47009, t47011, t47013, t47019, t47025, t47040, t47059, t47060, t47063, t47065, t47067)
}
