//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta497 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1774;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1775;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1776;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta497<F: Float>(t28840: F, t7296: F, t72: F, t8103: F, t686: F, t7284: F, t1398: F, t543: F, t8085: F, t7301: F, t26265: F, t5722: F, t14224: F, t26304: F, t7289: F, t26356: F, t26361: F, t26363: F, t27868: F, t28826: F, t28830: F, t28838: F, t7292: F, t7295: F, t7532: F, t7917: F, t8104: F, t1903: F, t7506: F, t27924: F, t27926: F, t27929: F, t25974: F, t25980: F, t25989: F, t25998: F, t26006: F, t26025: F, t26321: F, t26324: F, t26328: F, t27919: F, t27921: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t28841, t28844, t28845) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1774::<F>(t28840, t7296, t72, t8103, t686);
        let (t28846, t28850, t28853, t28855, t28858, t28861) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1775::<F>(t28845, t7284, t1398, t543, t8085, t7301, t26265, t5722, t14224, t26304, t7289, t26356, t26361, t26363, t27868, t28826, t28830, t28838, t28841, t7292, t7295, t7532, t7917, t8104);
        let (t28862, t28863, t28875) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1776::<F>(t1903, t7506, t7296, t27924, t27926, t27929, t25974, t25980, t25989, t25998, t26006, t26025, t26321, t26324, t26328, t27919, t27921);
    (t28841, t28844, t28845, t28846, t28850, t28853, t28855, t28858, t28861, t28862, t28863, t28875)
}
