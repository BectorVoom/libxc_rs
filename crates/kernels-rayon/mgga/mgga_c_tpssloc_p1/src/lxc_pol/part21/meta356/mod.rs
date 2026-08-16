//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta356 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1766;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1767;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1768;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1769;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1770;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1771;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta356(t13242: f64, t4180: f64, t4182: f64, t4181: f64, t9632: f64, t2642: f64, t4166: f64, t2617: f64, t4177: f64, t2628: f64, t836: f64, t812: f64, t4184: f64, t242: f64, t9972: f64, t2631: f64, t9975: f64, t13225: f64, t13231: f64, t13234: f64, t13237: f64, t2643: f64, t2649: f64, t4178: f64, t4191: f64, t4240: f64, t9639: f64, t9642: f64, t9668: f64, t9672: f64, t9675: f64, t9679: f64, t9986: f64, t9988: f64, t9994: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13244, t13248, t13251) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1766(t13242, t4180, t4182, t4181, t9632, t2642, t4166);
        let t13254 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1767(t2617, t4177);
        let (t13257, t13258) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1768(t2628, t836, t812);
        let (t13260, t13261, t13262) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1769(t13258, t4184, t242, t9972, t812);
        let t13263 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1770(t2631, t9975);
        let (t13265, t13268) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1771(t13263, t4180, t4181, t13225, t13231, t13234, t13237, t13244, t13248, t13251, t13254, t13260, t13262, t2643, t2649, t4178, t4184, t4191, t4240, t9639, t9642, t9668, t9672, t9675, t9679, t9986, t9988, t9994);
    (t13244, t13248, t13251, t13254, t13257, t13258, t13260, t13261, t13262, t13263, t13265, t13268)
}
