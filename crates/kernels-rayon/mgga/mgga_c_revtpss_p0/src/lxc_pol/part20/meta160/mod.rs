//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta160 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk864;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk865;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk866;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk867;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk868;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta160(t1121: f64, t1263: f64, t3362: f64, t3617: f64, t1012: f64, t1224: f64, t3698: f64, t3623: f64, t4890: f64, t3782: f64, t1248: f64, t471: f64, t3767: f64, t3603: f64, t1214: f64, t1260: f64, t3670: f64, t3627: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5296, t5302, t5308, t5312, t5330) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk864(t1121, t1263, t3362, t3617, t1012, t1224, t3698, t3623, t4890);
        let t5331 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk865(t3782, t5330);
        let (t5333, t5340) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk866(t1248, t471, t3767, t5330);
        let (t5341, t5352, t5384) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk867(t1248, t3603, t1214, t471, t1260, t3670);
        let t5405 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk868(t3627, t471);
    (t5296, t5302, t5308, t5312, t5330, t5331, t5333, t5340, t5341, t5352, t5384, t5405)
}
