//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta369 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1402;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1403;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta369(t4353: f64, t9794: f64, t10760: f64, t10890: f64, t1549: f64, t10811: f64, t4462: f64, t4416: f64, t808: f64, t10886: f64, t2703: f64, t4458: f64, t10769: f64, t828: f64, t1544: f64, t836: f64, t2746: f64, t2710: f64, t2713: f64, t4371: f64, t10744: f64, t10905: f64, t4442: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14760, t14761, t14765, t14777, t14780, t14783) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1402(t4353, t9794, t10760, t10890, t1549, t10811, t4462, t4416, t808, t10886, t2703, t4458);
        let (t14785, t14786, t14791, t14817, t14820, t14823) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1403(t10769, t828, t1544, t836, t2746, t2710, t2713, t4371, t4353, t808, t10744, t10905, t4442);
    (t14760, t14761, t14765, t14777, t14780, t14783, t14785, t14786, t14791, t14817, t14820, t14823)
}
