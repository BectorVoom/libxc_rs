//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta845 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3162;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3163;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3164;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta845<F: Float>(t3476: F, t5117: F, t12228: F, t16668: F, t44017: F, t16661: F, t3427: F, t3433: F, t12243: F, t16688: F, t3385: F, t5105: F, t12227: F, t5108: F, t3451: F, t3383: F, t5060: F, t3386: F, t12247: F, t1719: F, t12249: F, t1756: F, t3521: F, t43762: F, t43771: F, t43773: F, t43781: F, t43783: F, t43785: F, t43787: F, t43814: F, t43817: F, t56151: F, t56155: F, t56159: F, t56163: F, t56167: F, t58029: F, t58032: F, t58035: F, t58038: F, t58041: F, t58044: F, t58046: F, t58048: F, t58051: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t58317, t58322, t58325, t58327, t58330) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3162::<F>(t3476, t5117, t12228, t16668, t44017, t16661, t3427, t3433, t12243, t16688, t3385, t5105);
        let (t58333, t58336, t58341, t58344, t58345) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3163::<F>(t12227, t12228, t5108, t3451, t5117, t3383, t5060, t3386, t12247, t1719, t12249, t1756, t3521);
        let (t58359, t58372) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3164::<F>(t43762, t43771, t43773, t43781, t43783, t43785, t43787, t43814, t43817, t56151, t56155, t56159, t56163, t56167, t58029, t58032, t58035, t58038, t58041, t58044, t58046, t58048, t58051);
    (t58317, t58322, t58325, t58327, t58330, t58333, t58336, t58341, t58344, t58345, t58359, t58372)
}
