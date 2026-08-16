//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta497 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1774;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1775;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1776;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta497(t28840: f64, t7296: f64, t72: f64, t8103: f64, t686: f64, t7284: f64, t1398: f64, t543: f64, t8085: f64, t7301: f64, t26265: f64, t5722: f64, t14224: f64, t26304: f64, t7289: f64, t26356: f64, t26361: f64, t26363: f64, t27868: f64, t28826: f64, t28830: f64, t28838: f64, t7292: f64, t7295: f64, t7532: f64, t7917: f64, t8104: f64, t1903: f64, t7506: f64, t27924: f64, t27926: f64, t27929: f64, t25974: f64, t25980: f64, t25989: f64, t25998: f64, t26006: f64, t26025: f64, t26321: f64, t26324: f64, t26328: f64, t27919: f64, t27921: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28841, t28844, t28845) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1774(t28840, t7296, t72, t8103, t686);
        let (t28846, t28850, t28853, t28855, t28858, t28861) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1775(t28845, t7284, t1398, t543, t8085, t7301, t26265, t5722, t14224, t26304, t7289, t26356, t26361, t26363, t27868, t28826, t28830, t28838, t28841, t7292, t7295, t7532, t7917, t8104);
        let (t28862, t28863, t28875) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1776(t1903, t7506, t7296, t27924, t27926, t27929, t25974, t25980, t25989, t25998, t26006, t26025, t26321, t26324, t26328, t27919, t27921);
    (t28841, t28844, t28845, t28846, t28850, t28853, t28855, t28858, t28861, t28862, t28863, t28875)
}
