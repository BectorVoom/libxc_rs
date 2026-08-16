//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta232 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1372;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1373;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1374;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1375;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1376;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1377;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta232(t1353: f64, t5651: f64, t1394: f64, t5591: f64, t1392: f64, t1395: f64, t1877: f64, t1879: f64, t539: f64, t541: f64, t5644: f64, t5650: f64, t543: f64, t1390: f64, t828: f64, t1883: f64, t221: f64, t4019: f64, t4018: f64, t241: f64, t4000: f64, t820: f64, t550: f64, t72: f64, t245: f64, t125: f64, t1882: f64, t1398: f64, t4003: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5652, t5655, t5658) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1372(t1353, t5651, t1394, t5591, t1392, t1395, t1877, t1879, t539, t541, t5644, t5650);
        let t5659 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1373(t543, t5658);
        let (t5661, t5665, t5666, t5671) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1374(t1390, t5659, t828, t1883, t221, t4019, t4018, t241, t4000, t820);
        let (t5672, t5673) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1375(t550, t72, t245);
        let t5674 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1376(t125, t1882);
        let t5675 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1377(t1398, t4003);
    (t5652, t5655, t5658, t5659, t5661, t5665, t5666, t5671, t5672, t5673, t5674, t5675)
}
