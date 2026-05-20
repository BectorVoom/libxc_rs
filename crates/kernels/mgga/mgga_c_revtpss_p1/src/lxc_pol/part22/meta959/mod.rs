//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta959 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3217;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3218;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3219;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3220;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta959<F: Float>(t45: F, t39438: F, t49876: F, t11064: F, t6075: F, t37: F, t5940: F, t2612: F, t10446: F, t13312: F, t13396: F, t14401: F, t18272: F, t18277: F, t18281: F, t2251: F, t2258: F, t2375: F, t39825: F, t4377: F, t5819: F, t5825: F, t606: F, t60717: F, t60754: F, t78: F, zeta_threshold: F, t57: F, t10457: F, t14413: F, t18286: F, t18291: F, t2382: F, t39840: F, t4384: F, t81: F, t150: F, t190: F, t2609: F, t706: F, t18550: F, t72: F, t757: F, t162: F, t187: F, t49897: F, t4343: F, t890: F, t18871: F, t1940: F, t2403: F, t2408: F, t2832: F, t39442: F, t4556: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t61031, t61032, t61033, t61039, t61062) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3217::<F>(t45, t39438, t49876, t11064, t6075, t37, t5940, t2612, t10446, t13312, t13396, t14401, t18272, t18277, t18281, t2251, t2258, t2375, t39825, t4377, t5819, t5825, t606, t60717, t60754, t78, zeta_threshold);
        let t61085 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3218::<F>(t57, t10457, t13312, t13396, t14413, t18281, t18286, t18291, t2251, t2258, t2382, t39840, t4384, t5819, t5825, t606, t60717, t60754, t81, zeta_threshold);
        let (t61088, t61091, t61094, t61097) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3219::<F>(t61062, t61085, t150, t190, t2609, t5825, t706, t18550, t72, t757, t162, t187);
        let (t61101, t61106) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3220::<F>(t49897, t4343, t890, t18871, t1940, t2403, t2408, t2832, t39442, t4556, t61031, t61032, t61033, t61039, t61088, t61091, t61094, t61097);
    (t61031, t61032, t61039, t61088, t61091, t61094, t61097, t61101, t61106)
}
