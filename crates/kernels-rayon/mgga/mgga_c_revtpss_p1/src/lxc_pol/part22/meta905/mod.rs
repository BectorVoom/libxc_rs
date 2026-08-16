//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta905 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3103;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3104;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta905(t15752: f64, t16049: f64, t16087: f64, t53884: f64, t15988: f64, t3241: f64, t1011: f64, t15158: f64, t15987: f64, t15994: f64, t43537: f64, t53668: f64, t11933: f64, t16035: f64, t11774: f64, t127: f64, t15585: f64, t4872: f64, t16226: f64, t16229: f64, t53405: f64, t3230: f64, t4857: f64, t11817: f64, t4858: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54261, t54289, t54303, t54306, t54314, t54316) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3103(t15752, t16049, t16087, t53884, t15988, t3241, t1011, t15158, t15987, t15994, t43537, t53668);
        let (t54324, t54341, t54348, t54384, t54387) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3104(t11933, t16035, t11774, t127, t15585, t4872, t16226, t16229, t53405, t3230, t4857, t11817, t4858);
    (t54261, t54289, t54303, t54306, t54314, t54316, t54324, t54341, t54348, t54384, t54387)
}
