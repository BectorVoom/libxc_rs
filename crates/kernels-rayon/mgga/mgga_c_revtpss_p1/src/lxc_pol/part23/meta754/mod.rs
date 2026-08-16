//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta754 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2543;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2544;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta754(t11821: f64, t65: f64, t11144: f64, t11970: f64, t1660: f64, t27527: f64, t2852: f64, t11150: f64, t27531: f64, t127: f64, t4823: f64, t15690: f64, t247: f64, t42792: f64, t4757: f64, t4837: f64, t3091: f64, t43240: f64, t4782: f64, t41296: f64, t42471: f64, t3155: f64, t999: f64, t1011: f64, t4886: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53322, t53326, t53328, t53332, t53391, t53405) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2543(t11821, t65, t11144, t11970, t1660, t27527, t2852, t11150, t27531, t127, t4823, t15690);
        let (t53432, t53437, t53473, t53511, t53542) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2544(t247, t42792, t4757, t4837, t3091, t43240, t4782, t41296, t42471, t3155, t999, t1011, t4886, t697);
    (t53322, t53326, t53328, t53332, t53391, t53405, t53432, t53437, t53473, t53511, t53542)
}
