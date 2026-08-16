//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta299 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1188;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1189;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta299(t221: f64, t4019: f64, t4057: f64, t4018: f64, t1386: f64, t2681: f64, t820: f64, t1401: f64, t4000: f64, t843: f64, t4006: f64, t136: f64, t4011: f64, t3829: f64, t3978: f64, t3970: f64, t3989: f64, t4056: f64, t550: f64, t543: f64, t3992: f64, t2661: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9905, t9906, t9909, t9910, t9919, t9921) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1188(t221, t4019, t4057, t4018, t1386, t2681, t820, t1401, t4000, t843, t4006, t136, t4011);
        let (t9923, t9924, t9926, t9930, t9932, t9934) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1189(t221, t3829, t9921, t3978, t3970, t3989, t4056, t550, t543, t3992, t2661, t240, t4000);
    (t9905, t9906, t9909, t9910, t9919, t9921, t9923, t9924, t9926, t9930, t9932, t9934)
}
