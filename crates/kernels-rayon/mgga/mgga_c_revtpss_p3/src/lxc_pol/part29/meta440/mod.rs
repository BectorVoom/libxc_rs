//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta440 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1651;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1652;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta440(t2311: f64, t76: f64, t10298: f64, t38: f64, t2248: f64, t77: f64, t84: f64, t2247: f64, t607: f64, t1927: f64, t644: f64, t4144: f64, t9593: f64, t196: f64, t197: f64, t3821: f64, t2394: f64, t30: f64, t2411: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25146, t25150, t25159, t25162) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1651(t2311, t76, t10298, t38, t2248, t77, t84, t2247, t607);
        let (t25163, t25177, t25188, t25198, t25207) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1652(t1927, t644, t4144, t9593, t196, t197, t3821, t2394, t30, t2411);
    (t25146, t25150, t25159, t25162, t25163, t25177, t25188, t25198, t25207)
}
