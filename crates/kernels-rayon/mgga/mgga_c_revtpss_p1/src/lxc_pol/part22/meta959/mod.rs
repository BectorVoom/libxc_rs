//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta959 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3217;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3218;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3219;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3220;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta959(t45: f64, t39438: f64, t49876: f64, t11064: f64, t6075: f64, t37: f64, t5940: f64, t2612: f64, t10446: f64, t13312: f64, t13396: f64, t14401: f64, t18272: f64, t18277: f64, t18281: f64, t2251: f64, t2258: f64, t2375: f64, t39825: f64, t4377: f64, t5819: f64, t5825: f64, t606: f64, t60717: f64, t60754: f64, t78: f64, zeta_threshold: f64, t57: f64, t10457: f64, t14413: f64, t18286: f64, t18291: f64, t2382: f64, t39840: f64, t4384: f64, t81: f64, t150: f64, t190: f64, t2609: f64, t706: f64, t18550: f64, t72: f64, t757: f64, t162: f64, t187: f64, t49897: f64, t4343: f64, t890: f64, t18871: f64, t1940: f64, t2403: f64, t2408: f64, t2832: f64, t39442: f64, t4556: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61031, t61032, t61033, t61039, t61062) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3217(t45, t39438, t49876, t11064, t6075, t37, t5940, t2612, t10446, t13312, t13396, t14401, t18272, t18277, t18281, t2251, t2258, t2375, t39825, t4377, t5819, t5825, t606, t60717, t60754, t78, zeta_threshold);
        let t61085 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3218(t57, t10457, t13312, t13396, t14413, t18281, t18286, t18291, t2251, t2258, t2382, t39840, t4384, t5819, t5825, t606, t60717, t60754, t81, zeta_threshold);
        let (t61088, t61091, t61094, t61097) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3219(t61062, t61085, t150, t190, t2609, t5825, t706, t18550, t72, t757, t162, t187);
        let (t61101, t61106) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3220(t49897, t4343, t890, t18871, t1940, t2403, t2408, t2832, t39442, t4556, t61031, t61032, t61033, t61039, t61088, t61091, t61094, t61097);
    (t61031, t61032, t61039, t61088, t61091, t61094, t61097, t61101, t61106)
}
