//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta153 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk750;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk751;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta153(t225: f64, t4075: f64, t1429: f64, t2435: f64, t1428: f64, t2777: f64, t2439: f64, t1385: f64, t1398: f64, t555: f64, t543: f64, t2782: f64, t1419: f64, t545: f64, t869: f64, t689: f64, t136: f64, t2457: f64, t3964: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4076, t4082, t4083, t4085, t4086) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk750(t225, t4075, t1429, t2435, t1428, t2777, t2439, t1385);
        let (t4089, t4090, t4092, t4093, t4094, t4096, t4099) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk751(t1398, t555, t4086, t543, t2782, t1419, t545, t869, t689, t136, t2457, t3964);
    (t4076, t4082, t4083, t4085, t4086, t4089, t4090, t4092, t4093, t4094, t4096, t4099)
}
