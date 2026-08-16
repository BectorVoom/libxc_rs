//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta903 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2891;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2892;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2893;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2894;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2895;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2896;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2897;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta903(t23842: f64, t606: f64, t51957: f64, t51958: f64, t51963: f64, t52110: f64, t15129: f64, t5825: f64, t128: f64, t2850: f64, t18281: f64, t4573: f64, t23481: f64, t904: f64, t4186: f64, t6092: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t77513 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2891(t23842, t606);
        let t77515 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2892(t51957, t51958, t77513);
        let t77518 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2893(t51957, t51963, t77513);
        let t77521 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2894(t51957, t52110, t77513);
        let (t77525, t77527) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2895(t15129, t5825, t128, t2850);
        let (t77529, t77531) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2896(t18281, t4573, t128, t2850);
        let (t77533, t77535) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2897(t23481, t606, t128, t904);
        let (t77537, t77539) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2898(t4186, t6092, t128, t904);
    (t77513, t77515, t77518, t77521, t77525, t77527, t77529, t77531, t77533, t77535, t77537, t77539)
}
