//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1209;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1210;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1211;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1212;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1213;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1214;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta270<F: Float>(t3: F, t7690: F, t1461: F, t2170: F, t573: F, t7329: F, t7333: F, t7336: F, t38: F, t4173: F, t1497: F, t84: F, param_d: F, t77: F, t1470: F, t603: F, t1493: F, t76: F, t1937: F, t4248: F, t1518: F, t94: F, t1843: F, t1936: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7691, t7696, t7700, t7702, t7705) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1209::<F>(t3, t7690, t1461, t2170, t573, t7329, t7333, t7336, t38, t4173, t1497, t84, param_d);
        let t7706 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1210::<F>(t77, t7705);
        let t7709 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1211::<F>(t1470, t603);
        let t7719 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1212::<F>(t1493, t76);
        let (t7731, t7732) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1213::<F>(t1937, t4248, t1518, t94);
        let (t7734, t7735) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1214::<F>(t1937, t7732, t1843, t1936);
    (t7691, t7696, t7700, t7702, t7705, t7706, t7709, t7719, t7731, t7732, t7734, t7735)
}
