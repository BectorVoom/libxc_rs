//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta284 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1518;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1519;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1520;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta284(t10227: f64, t10228: f64, t2349: f64, t658: f64, t2256: f64, t9343: f64, t100: f64, t106: f64, t107: f64, t2358: f64, t661: f64, t2357: f64, t2362: f64, t108: f64, t101: f64, t10217: f64, t105: f64, t2344: f64, t2351: f64, t2354: f64, t656: f64, t659: f64, t97: f64, t114: f64, t655: f64, t10201: f64, t10202: f64, t10204: f64, t10206: f64, t10210: f64, t10214: f64, t69: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10229, t10233, t10236, t10237, t10241, t10242, t10243, t10246) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1518(t10227, t10228, t2349, t658, t2256, t9343, t100, t106, t107, t2358, t661, t2357);
        let (t10247, t10250, t10251, t10254) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1519(t10246, t2362, t10236, t108, t101, t10217, t10229, t10233, t10237, t10243, t105, t2344, t2351, t2354, t656, t659, t97);
        let (t10255, t10259) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1520(t114, t10254, t655, t10201, t10202, t10204, t10206, t10210, t10214, t69);
    (t10236, t10241, t10242, t10243, t10247, t10250, t10251, t10254, t10255, t10259)
}
