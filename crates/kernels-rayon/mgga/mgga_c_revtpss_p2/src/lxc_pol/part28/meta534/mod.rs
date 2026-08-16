//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta534 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1976;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1977;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta534(t4181: f64, t603: f64, t4187: f64, t38: f64, t7714: f64, t2247: f64, t1493: f64, t644: f64, t77: f64, t13272: f64, t6957: f64, t4173: f64, t607: f64, t7705: f64, t1497: f64, t1927: f64, t1926: f64, t1470: f64, t1928: f64, t25099: f64, t25157: f64, t25162: f64, t25164: f64, t6958: f64, t6960: f64, t6963: f64, t6974: f64, t6978: f64, t7706: f64, t7709: f64, t7716: f64, t7720: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28116, t28119, t28126, t28127, t28133, t28138, t28141) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1976(t4181, t603, t4187, t38, t7714, t2247, t1493, t644, t77, t13272, t6957, t4173, t607);
        let (t28147, t28150, t28151, t28154, t28157) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1977(t644, t77, t7705, t1497, t1927, t1926, t1470, t2247, t1928, t25099, t25157, t25162, t25164, t28116, t28119, t28127, t28133, t28138, t28141, t6958, t6960, t6963, t6974, t6978, t7706, t7709, t7716, t7720);
    (t28116, t28119, t28126, t28127, t28133, t28138, t28141, t28147, t28150, t28151, t28154, t28157)
}
