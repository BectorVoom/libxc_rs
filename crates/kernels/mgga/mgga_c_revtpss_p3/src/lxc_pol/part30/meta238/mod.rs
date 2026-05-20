//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta238 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1072;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1073;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1074;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1075;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta238<F: Float>(t482: F, t5245: F, t371: F, t372: F, t1234: F, t1803: F, t225: F, t5219: F, t480: F, t3623: F, t4890: F, t3782: F, t1794: F, t3153: F, t1248: F, t471: F, t3720: F, t1222: F, t1235: F, t1238: F, t1252: F, t1261: F, t1791: F, t3637: F, t3667: F, t3711: F, t5293: F, t5299: F, t5304: F, t5309: F, t5313: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5318, t5320, t5323, t5326) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1072::<F>(t482, t5245, t371, t372, t1234, t1803, t225, t5219);
        let (t5327, t5330) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1073::<F>(t480, t5326, t3623, t4890);
        let (t5331, t5332) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1074::<F>(t3782, t5330, t1794, t3153);
        let (t5333, t5334, t5335, t5338) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1075::<F>(t1248, t471, t5332, t3720, t1222, t1235, t1238, t1252, t1261, t1791, t3637, t3667, t3711, t5293, t5299, t5304, t5309, t5313, t5320, t5323, t5327, t5331);
    (t5318, t5320, t5323, t5326, t5327, t5330, t5331, t5332, t5333, t5334, t5335, t5338)
}
