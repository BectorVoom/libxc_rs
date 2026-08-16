//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1751;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1752;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta460(t177: f64, t762: f64, t9363: f64, t1340: f64, t40135: f64, t4038: f64, t9425: f64, t1330: f64, t512: f64, t9544: f64, t3869: f64, t39739: f64, t39430: f64, t9572: f64, t9860: f64, t39742: f64, t39440: f64, t9866: f64, t9863: f64, t40072: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47107, t47109, t47111, t47114, t47116) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1751(t177, t762, t9363, t1340, t40135, t4038, t9425, t1330, t512, t9544, t3869, t39739);
        let (t47118, t47120, t47122, t47124, t47126, t47128, t47129) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1752(t3869, t39430, t9572, t9860, t39742, t39440, t9866, t9863, t40072, t47107, t47109, t47111, t47114, t47116);
    (t47107, t47109, t47111, t47114, t47116, t47118, t47120, t47122, t47124, t47126, t47128, t47129)
}
