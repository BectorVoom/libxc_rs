//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta309 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1582;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1583;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1584;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1585;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1586;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta309(t1284: f64, t3555: f64, t3624: f64, t1121: f64, t3603: f64, t606: f64, t221: f64, t462: f64, t68: f64, t461: f64, t1209: f64, t3766: f64, t5330: f64, t1214: f64, t11772: f64, t3623: f64, t3717: f64, t1263: f64, t675: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12831, t12832, t12839, t12840, t12851, t12853, t12854) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1582(t1284, t3555, t3624, t1121, t3603, t606, t221, t462, t68, t461, t1209, t3766);
        let t12855 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1583(t12854, t5330);
        let (t12856, t12865) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1584(t1214, t3603, t11772, t3623);
        let t12866 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1585(t12865, t3717);
        let t12879 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1586(t1263, t675);
    (t12831, t12832, t12839, t12840, t12851, t12853, t12854, t12855, t12856, t12865, t12866, t12879)
}
