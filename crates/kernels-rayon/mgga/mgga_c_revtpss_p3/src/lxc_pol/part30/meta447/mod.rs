//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1707;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1708;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1709;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1710;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta447(t17353: f64, t17514: f64, t1802: f64, t3147: f64, t3597: f64, t3594: f64, t1244: f64, t1214: f64, t4186: f64, t5296: f64, t1042: f64, t1469: f64, t3584: f64, t3172: f64, t5286: f64, t1247: f64, t3707: f64, t5292: f64, t12268: f64, t3617: f64, t15936: f64, t3708: f64, t5265: f64, t13392: f64, t5302: f64, t1252: f64, t1261: f64, t12956: f64, t3591: f64, t3606: f64, t3613: f64, t3711: f64, t5293: f64, t5299: f64, t1260: f64, t5326: f64, t17376: f64, t3599: f64, t17482: f64, t3604: f64, t3720: f64, t3372: f64, t5277: f64, t12855: f64, t12964: f64, t12979: f64, t12985: f64, t12996: f64, t3620: f64, t3640: f64, t3714: f64, t5381: f64, t5391: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17515, t17523, t17525, t17529, t17536, t17539) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1707(t17353, t17514, t1802, t3147, t3597, t3594, t1244, t1214, t4186, t5296, t1042, t1469, t3584);
        let (t17541, t17544, t17546, t17547, t17552, t17556) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1708(t17539, t5296, t1042, t3172, t5286, t1247, t3707, t5292, t12268, t3617, t15936, t3708, t5265);
        let (t17558, t17561) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1709(t13392, t5302, t1042, t1252, t1261, t12956, t17525, t17529, t17536, t17541, t17546, t17547, t17552, t17556, t3591, t3606, t3613, t3711, t5293, t5299);
        let (t17580, t17584, t17587) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1710(t1260, t5326, t17376, t3599, t17482, t3604, t3720, t3372, t5277, t1042, t12855, t12964, t12979, t12985, t12996, t3606, t3620, t3640, t3711, t3714, t5381, t5391);
    (t17515, t17523, t17536, t17541, t17544, t17552, t17558, t17561, t17580, t17584, t17587)
}
