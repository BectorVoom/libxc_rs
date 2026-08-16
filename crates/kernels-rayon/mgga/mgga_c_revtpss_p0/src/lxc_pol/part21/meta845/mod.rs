//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta845 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3162;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3163;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3164;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta845(t3476: f64, t5117: f64, t12228: f64, t16668: f64, t44017: f64, t16661: f64, t3427: f64, t3433: f64, t12243: f64, t16688: f64, t3385: f64, t5105: f64, t12227: f64, t5108: f64, t3451: f64, t3383: f64, t5060: f64, t3386: f64, t12247: f64, t1719: f64, t12249: f64, t1756: f64, t3521: f64, t43762: f64, t43771: f64, t43773: f64, t43781: f64, t43783: f64, t43785: f64, t43787: f64, t43814: f64, t43817: f64, t56151: f64, t56155: f64, t56159: f64, t56163: f64, t56167: f64, t58029: f64, t58032: f64, t58035: f64, t58038: f64, t58041: f64, t58044: f64, t58046: f64, t58048: f64, t58051: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58317, t58322, t58325, t58327, t58330) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3162(t3476, t5117, t12228, t16668, t44017, t16661, t3427, t3433, t12243, t16688, t3385, t5105);
        let (t58333, t58336, t58341, t58344, t58345) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3163(t12227, t12228, t5108, t3451, t5117, t3383, t5060, t3386, t12247, t1719, t12249, t1756, t3521);
        let (t58359, t58372) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3164(t43762, t43771, t43773, t43781, t43783, t43785, t43787, t43814, t43817, t56151, t56155, t56159, t56163, t56167, t58029, t58032, t58035, t58038, t58041, t58044, t58046, t58048, t58051);
    (t58317, t58322, t58325, t58327, t58330, t58333, t58336, t58341, t58344, t58345, t58359, t58372)
}
