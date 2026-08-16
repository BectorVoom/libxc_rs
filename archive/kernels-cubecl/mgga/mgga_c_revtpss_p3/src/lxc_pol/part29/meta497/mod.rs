//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta497 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1806;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1807;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1808;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1809;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1810;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta497<F: Float>(t28840: F, t7296: F, t72: F, t8103: F, t686: F, t7284: F, t1398: F, t543: F, t8085: F, t7301: F, t26265: F, t5722: F, t14224: F, t26304: F, t7289: F, t26356: F, t26361: F, t26363: F, t27868: F, t28826: F, t28830: F, t28838: F, t7292: F, t7295: F, t7532: F, t7917: F, t8104: F, t1903: F, t7506: F, t27924: F, t27926: F, t27929: F, t25974: F, t25980: F, t25989: F, t25998: F, t26006: F, t26025: F, t26321: F, t26324: F, t26328: F, t27919: F, t27921: F, t27937: F, t27955: F, t26016: F, t26310: F, t26312: F, t26325: F, t27933: F, t27941: F, t27943: F, t27945: F, t27947: F, t27949: F, t27951: F, t27953: F, t27957: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t28841, t28844, t28845, t28846, t28850, t28853) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1806::<F>(t28840, t7296, t72, t8103, t686, t7284, t1398, t543, t8085, t7301, t26265, t5722);
        let (t28855, t28861) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1807::<F>(t14224, t26304, t28845, t7289, t26356, t26361, t26363, t27868, t28826, t28830, t28838, t28841, t28846, t28850, t28853, t7292, t7295, t7532, t7917, t8104);
        let (t28862, t28863, t28875) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1808::<F>(t1903, t7506, t7296, t27924, t27926, t27929, t25974, t25980, t25989, t25998, t26006, t26025, t26321, t26324, t26328, t27919, t27921);
        let t28887 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1809::<F>(t27937, t27955, t26016, t26310, t26312, t26325, t27933, t27941, t27943, t27945, t27947, t27949, t27951, t27953, t27957);
        let t28888 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1810::<F>(t28875, t28887);
    (t28841, t28844, t28845, t28850, t28855, t28861, t28862, t28863, t28888)
}
