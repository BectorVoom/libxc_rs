//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta331 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1251;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1252;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta331(t12865: f64, t3717: f64, t1263: f64, t675: f64, t1122: f64, t247: f64, t1261: f64, t126: f64, t3617: f64, t1231: f64, t3655: f64, t2434: f64, t371: f64, t482: f64, t481: f64, t11262: f64, t1251: f64, t1247: f64, t1284: f64, t3566: f64, t3624: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12866, t12879, t12882, t12884, t12893, t12898) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1251(t12865, t3717, t1263, t675, t1122, t247, t1261, t126, t3617, t1231, t3655, t2434, t371, t482);
        let (t12900, t12905, t12910, t12915, t12916) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1252(t12898, t481, t11262, t1251, t1247, t1284, t3566, t3624, t126, t482, t828);
    (t12866, t12879, t12882, t12884, t12893, t12900, t12905, t12910, t12915, t12916)
}
