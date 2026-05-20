//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1707;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1708;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1709;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1710;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta447<F: Float>(t17353: F, t17514: F, t1802: F, t3147: F, t3597: F, t3594: F, t1244: F, t1214: F, t4186: F, t5296: F, t1042: F, t1469: F, t3584: F, t3172: F, t5286: F, t1247: F, t3707: F, t5292: F, t12268: F, t3617: F, t15936: F, t3708: F, t5265: F, t13392: F, t5302: F, t1252: F, t1261: F, t12956: F, t3591: F, t3606: F, t3613: F, t3711: F, t5293: F, t5299: F, t1260: F, t5326: F, t17376: F, t3599: F, t17482: F, t3604: F, t3720: F, t3372: F, t5277: F, t12855: F, t12964: F, t12979: F, t12985: F, t12996: F, t3620: F, t3640: F, t3714: F, t5381: F, t5391: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17515, t17523, t17525, t17529, t17536, t17539) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1707::<F>(t17353, t17514, t1802, t3147, t3597, t3594, t1244, t1214, t4186, t5296, t1042, t1469, t3584);
        let (t17541, t17544, t17546, t17547, t17552, t17556) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1708::<F>(t17539, t5296, t1042, t3172, t5286, t1247, t3707, t5292, t12268, t3617, t15936, t3708, t5265);
        let (t17558, t17561) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1709::<F>(t13392, t5302, t1042, t1252, t1261, t12956, t17525, t17529, t17536, t17541, t17546, t17547, t17552, t17556, t3591, t3606, t3613, t3711, t5293, t5299);
        let (t17580, t17584, t17587) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1710::<F>(t1260, t5326, t17376, t3599, t17482, t3604, t3720, t3372, t5277, t1042, t12855, t12964, t12979, t12985, t12996, t3606, t3620, t3640, t3711, t3714, t5381, t5391);
    (t17515, t17523, t17536, t17541, t17544, t17552, t17558, t17561, t17580, t17584, t17587)
}
