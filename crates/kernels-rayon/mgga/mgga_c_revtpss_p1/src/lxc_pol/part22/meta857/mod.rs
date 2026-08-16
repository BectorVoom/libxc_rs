//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta857 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3003;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta857(t14701: f64, t40731: f64, t14468: f64, t221: f64, t2674: f64, t2675: f64, t14662: f64, t231: f64, t243: f64, t2661: f64, t2662: f64, t14648: f64, t14832: f64, t2430: f64, t10777: f64, t10779: f64, t14671: f64, t14872: f64, t10811: f64, t14682: f64, t14804: f64, t14923: f64, t4457: f64, t837: f64, t14853: f64, t2652: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50298, t50303, t50308, t50312) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3003(t14701, t40731, t14468, t221, t2674, t2675, t14662, t231, t243, t2661, t2662, t14648, t14832, t2430);
        let (t50325, t50328, t50347, t50351, t50353) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3004(t10777, t10779, t14671, t14872, t10811, t14682, t14804, t14923, t4457, t837, t14853, t2652);
    (t50298, t50303, t50308, t50312, t50325, t50328, t50347, t50351, t50353)
}
