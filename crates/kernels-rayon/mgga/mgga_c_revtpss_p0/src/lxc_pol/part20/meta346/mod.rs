//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta346 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1273;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1274;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta346(t12712: f64, t3629: f64, t12702: f64, t5330: f64, t12744: f64, t1214: f64, t5341: f64, t1250: f64, t140: f64, t3698: f64, t1012: f64, t13026: f64, t12268: f64, t3617: f64, t2258: f64, t3628: f64, t3367: f64, t471: f64, t2251: f64, t17350: f64, t3767: f64, t1121: f64, t1248: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17354, t17426, t17429, t17454, t17459, t17471, t17475) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1273(t12712, t3629, t12702, t5330, t12744, t1214, t5341, t1250, t140, t3698, t1012, t13026);
        let (t17550, t17638, t17644, t17654, t17656) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1274(t12268, t3617, t2258, t3628, t3367, t471, t2251, t17350, t3767, t1121, t1248, t606);
    (t17354, t17426, t17429, t17454, t17459, t17471, t17475, t17550, t17638, t17644, t17654, t17656)
}
