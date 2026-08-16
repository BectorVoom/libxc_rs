//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1493;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1494;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1495;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1496;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1497;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta273(t4012: f64, t828: f64, t9984: f64, t1384: f64, t235: f64, t239: f64, t820: f64, t4003: f64, t543: f64, t9898: f64, t1390: f64, t2482: f64, t27: f64, t4000: f64, t221: f64, t4004: f64, t4019: f64, t1410: f64, t3934: f64, t3944: f64, t9932: f64, t9937: f64, t9944: f64, t9953: f64, t9958: f64, t9963: f64, t9966: f64, t9971: f64, t9973: f64, t9977: f64, t9982: f64, t9755: f64, t9824: f64, t9928: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9986, t9989, t9990, t9991) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1493(t4012, t828, t9984, t1384, t235);
        let (t9993, t9994) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1494(t239, t820, t9991, t4003, t543);
        let t9995 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1495(t9898, t9994);
        let (t9997, t10001, t10003, t10004, t10006) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1496(t1390, t828, t9995, t2482, t27, t4000, t221, t4004, t4019, t1410, t3934, t3944, t9932, t9937, t9944, t9953, t9958, t9963, t9966, t9971, t9973, t9977, t9982, t9986, t9993);
        let t10008 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1497(t10006, t9755, t9824, t9928);
    (t9986, t9989, t9990, t9991, t9994, t9995, t9997, t10001, t10003, t10004, t10008)
}
