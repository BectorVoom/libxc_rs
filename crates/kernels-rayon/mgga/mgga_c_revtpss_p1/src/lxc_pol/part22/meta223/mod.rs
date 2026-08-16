//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta223 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1418;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1419;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1420;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1421;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1422;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1423;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1424;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta223(t1294: f64, t1828: f64, t3737: f64, t1284: f64, t1770: f64, t1280: f64, t5230: f64, t1287: f64, t5346: f64, t1774: f64, t3759: f64, t5245: f64, t354: f64, t471: f64, t1214: f64, t5351: f64, t3766: f64, t487: f64, t460: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5428, t5429) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1418(t1294, t1828, t3737);
        let t5436 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1419(t1284, t1770);
        let (t5443, t5446) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1420(t1280, t5230, t1287, t5346);
        let (t5449, t5452, t5457, t5458) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1421(t1774, t3759, t1280, t5245, t354, t471, t1214);
        let t5459 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1422(t5351, t5458);
        let t5462 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1423(t3766, t487);
        let t5463 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1424(t460, t5462);
    (t5428, t5429, t5436, t5443, t5446, t5449, t5452, t5457, t5458, t5459, t5462, t5463)
}
