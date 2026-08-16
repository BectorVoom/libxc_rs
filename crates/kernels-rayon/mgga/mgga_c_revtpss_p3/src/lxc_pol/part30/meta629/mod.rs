//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2193;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2194;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta629(t1497: f64, t2311: f64, t77: f64, t4241: f64, t640: f64, t13420: f64, t84: f64, t10298: f64, t1470: f64, t2242: f64, t4181: f64, t4187: f64, t28108: f64, t644: f64, t2315: f64, t7705: f64, t6977: f64, t1927: f64, t7719: f64, t13272: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t101172, t101176, t101182, t101187, t101190, t101193) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2193(t1497, t2311, t77, t4241, t640, t13420, t84, t10298, t1470, t2242, t4181, t4187);
        let (t101200, t101204, t101214, t101218, t101226, t101230) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2194(t28108, t644, t77, t2315, t7705, t1497, t6977, t1927, t4241, t7719, t13272, t607);
    (t101172, t101176, t101182, t101187, t101190, t101193, t101200, t101204, t101214, t101218, t101226, t101230)
}
