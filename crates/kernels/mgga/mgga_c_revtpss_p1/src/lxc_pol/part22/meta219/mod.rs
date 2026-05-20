//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta219 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1383;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1384;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1385;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1386;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1387;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1388;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1389;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1390;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1391;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1392;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta219<F: Float>(t225: F, t5219: F, t480: F, t3623: F, t4890: F, t3782: F, t1794: F, t3153: F, t1248: F, t471: F, t3720: F, t1222: F, t1235: F, t1238: F, t1252: F, t1261: F, t1791: F, t3637: F, t3667: F, t3711: F, t5293: F, t5299: F, t5304: F, t5309: F, t5313: F, t5320: F, t5323: F, t3767: F, t3603: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t5326 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1383::<F>(t225, t5219);
        let t5327 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1384::<F>(t480, t5326);
        let t5330 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1385::<F>(t3623, t4890);
        let t5331 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1386::<F>(t3782, t5330);
        let t5332 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1387::<F>(t1794, t3153);
        let t5333 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1388::<F>(t1248, t471);
        let (t5334, t5335) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1389::<F>(t5332, t5333, t3720);
        let t5338 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1390::<F>(t1222, t1235, t1238, t1252, t1261, t1791, t3637, t3667, t3711, t5293, t5299, t5304, t5309, t5313, t5320, t5323, t5327, t5331, t5335);
        let t5340 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1391::<F>(t3767, t5330);
        let t5341 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1392::<F>(t1248, t3603);
    (t5326, t5327, t5330, t5331, t5332, t5333, t5334, t5335, t5338, t5340, t5341)
}
