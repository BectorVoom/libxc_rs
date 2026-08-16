//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2092;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2093;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta637(t15654: f64, t1976: f64, t27708: f64, t3336: f64, t11108: f64, t7840: f64, t33: f64, t41154: f64, t1711: f64, t2411: f64, t28150: f64, t6973: f64, t1497: f64, t6977: f64, t1926: f64, t1927: f64, t4241: f64, t25163: f64, t7715: f64, t644: f64, t7719: f64, t13272: f64, t607: f64, t10301: f64, t1470: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t100760, t100802, t100806, t100981, t100987, t101211) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2092(t15654, t1976, t27708, t3336, t11108, t7840, t33, t41154, t1711, t2411, t28150, t6973);
        let (t101215, t101219, t101222, t101227, t101230, t101237) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2093(t1497, t6977, t1926, t1927, t4241, t25163, t7715, t644, t7719, t13272, t607, t10301, t1470);
    (t100760, t100802, t100806, t100981, t100987, t101211, t101215, t101219, t101222, t101227, t101230, t101237)
}
