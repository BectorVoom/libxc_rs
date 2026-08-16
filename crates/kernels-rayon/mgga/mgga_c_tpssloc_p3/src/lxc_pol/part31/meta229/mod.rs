//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta229 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk971;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk972;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk973;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta229(t1137: f64, t6052: f64, t3359: f64, t6036: f64, t3363: f64, t4721: f64, t5973: f64, t5977: f64, t5981: f64, t449: f64, t1694: f64, t1156: f64, t3383: f64, t3390: f64, t4770: f64, t5993: f64, t6000: f64, t6006: f64, t6008: f64, t6012: f64, t6015: f64, t6018: f64, t3403: f64, t1129: f64, t1148: f64, t1683: f64, t1695: f64, t3332: f64, t3357: f64, t3376: f64, t3401: f64, t436: f64, t4797: f64, t4835: f64, t5985: f64, t5987: f64, t5991: f64, t6023: f64, t6026: f64, t6031: f64, t6037: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6053, t6056, t6063, t6064, t6068) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk971(t1137, t6052, t3359, t6036, t3363, t4721, t5973, t5977, t5981, t449, t1694);
        let (t6069, t6084) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk972(t1156, t6068, t3383, t3390, t4721, t4770, t5973, t5977, t5981, t5993, t6000, t6006, t6008, t6012, t6015, t6018);
        let (t6085, t6088, t6091) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk973(t1156, t6084, t3403, t6068, t1129, t1148, t1683, t1695, t3332, t3357, t3376, t3401, t436, t4797, t4835, t5985, t5987, t5991, t6023, t6026, t6031, t6037, t6053, t6056, t6064, t6069);
    (t6053, t6056, t6063, t6064, t6068, t6069, t6084, t6085, t6088, t6091)
}
