//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1212;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1213;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1214;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1215;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta274<F: Float>(t670: F, t7226: F, t7228: F, t7230: F, t7584: F, t7586: F, t118: F, t1310: F, t1453: F, t2127: F, t2163: F, t2165: F, t508: F, t569: F, t649: F, t651: F, t671: F, t6990: F, t6992: F, t6995: F, t7005: F, t7236: F, t7241: F, t7314: F, t7317: F, t7591: F, t7683: F, t3: F, param_d: F, t1461: F, t2170: F, t573: F, t7329: F, t7333: F, t7336: F, t38: F, t4173: F, t1497: F, t84: F, t77: F, t1470: F, t603: F, t1493: F, t76: F, t1937: F, t4248: F, t1518: F, t94: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7687, t7690) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1212::<F>(t670, t7226, t7228, t7230, t7584, t7586, t118, t1310, t1453, t2127, t2163, t2165, t508, t569, t649, t651, t671, t6990, t6992, t6995, t7005, t7236, t7241, t7314, t7317, t7591, t7683);
        let (t7691, t7696) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1213::<F>(t3, t7690, param_d);
        let (t7700, t7702, t7705, t7706) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1214::<F>(t1461, t2170, t573, t7329, t7333, t7336, t7696, t38, t4173, t1497, t84, t77);
        let (t7709, t7719, t7731, t7732) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1215::<F>(t1470, t603, t1493, t76, t1937, t4248, t1518, t94);
    (t7687, t7690, t7691, t7696, t7700, t7702, t7705, t7706, t7709, t7719, t7731, t7732)
}
