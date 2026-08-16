//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1302;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1303;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1304;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta311(t1432: f64, t1433: f64, t9288: f64, t4066: f64, t72: f64, t686: f64, t136: f64, t1419: f64, t2457: f64, t3964: f64, t225: f64, t9646: f64, t1428: f64, t22: f64, t2452: f64, t557: f64, t10024: f64, t268: f64, t543: f64, t4101: f64, t1429: f64, t9292: f64, t4096: f64, t9285: f64, t1385: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10102, t10105, t10109, t10111) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1302(t1432, t1433, t9288, t4066, t72, t686, t136, t1419, t2457, t3964, t225, t9646);
        let (t10114, t10115) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1303(t10111, t1428, t22, t2452);
        let (t10117, t10120, t10126, t10129, t10130) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1304(t10115, t557, t10024, t268, t543, t4101, t1429, t9292, t3964, t4096, t9285, t1385, t4066);
    (t10102, t10105, t10109, t10111, t10114, t10115, t10117, t10120, t10126, t10129, t10130)
}
