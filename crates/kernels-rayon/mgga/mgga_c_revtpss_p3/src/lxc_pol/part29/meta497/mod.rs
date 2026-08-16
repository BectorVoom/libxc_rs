//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta497 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1806;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1807;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1808;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1809;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1810;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta497(t28840: f64, t7296: f64, t72: f64, t8103: f64, t686: f64, t7284: f64, t1398: f64, t543: f64, t8085: f64, t7301: f64, t26265: f64, t5722: f64, t14224: f64, t26304: f64, t7289: f64, t26356: f64, t26361: f64, t26363: f64, t27868: f64, t28826: f64, t28830: f64, t28838: f64, t7292: f64, t7295: f64, t7532: f64, t7917: f64, t8104: f64, t1903: f64, t7506: f64, t27924: f64, t27926: f64, t27929: f64, t25974: f64, t25980: f64, t25989: f64, t25998: f64, t26006: f64, t26025: f64, t26321: f64, t26324: f64, t26328: f64, t27919: f64, t27921: f64, t27937: f64, t27955: f64, t26016: f64, t26310: f64, t26312: f64, t26325: f64, t27933: f64, t27941: f64, t27943: f64, t27945: f64, t27947: f64, t27949: f64, t27951: f64, t27953: f64, t27957: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28841, t28844, t28845, t28846, t28850, t28853) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1806(t28840, t7296, t72, t8103, t686, t7284, t1398, t543, t8085, t7301, t26265, t5722);
        let (t28855, t28861) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1807(t14224, t26304, t28845, t7289, t26356, t26361, t26363, t27868, t28826, t28830, t28838, t28841, t28846, t28850, t28853, t7292, t7295, t7532, t7917, t8104);
        let (t28862, t28863, t28875) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1808(t1903, t7506, t7296, t27924, t27926, t27929, t25974, t25980, t25989, t25998, t26006, t26025, t26321, t26324, t26328, t27919, t27921);
        let t28887 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1809(t27937, t27955, t26016, t26310, t26312, t26325, t27933, t27941, t27943, t27945, t27947, t27949, t27951, t27953, t27957);
        let t28888 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1810(t28875, t28887);
    (t28841, t28844, t28845, t28850, t28855, t28861, t28862, t28863, t28888)
}
