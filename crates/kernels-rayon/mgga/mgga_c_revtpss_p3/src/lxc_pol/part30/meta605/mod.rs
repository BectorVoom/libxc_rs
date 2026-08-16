//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta605 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2067;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2068;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta605(t3572: f64, t8945: f64, t12657: f64, t2142: f64, t45551: f64, t473: f64, t1243: f64, t2149: f64, t37885: f64, t3555: f64, t7627: f64, t1209: f64, t26884: f64, t26921: f64, t7648: f64, t3552: f64, t26983: f64, t7658: f64, t12627: f64, t7635: f64, t27033: f64, t3801: f64, t12587: f64, t7669: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97363, t97370, t97377, t97397, t97402, t97419) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2067(t3572, t8945, t12657, t2142, t45551, t473, t1243, t2149, t37885, t3555, t7627, t1209, t26884);
        let (t97422, t97425, t97453, t97475, t97487, t97491) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2068(t26921, t7648, t2142, t3552, t26983, t7658, t12627, t7635, t27033, t3801, t12587, t7669);
    (t97363, t97370, t97377, t97397, t97402, t97419, t97422, t97425, t97453, t97475, t97487, t97491)
}
