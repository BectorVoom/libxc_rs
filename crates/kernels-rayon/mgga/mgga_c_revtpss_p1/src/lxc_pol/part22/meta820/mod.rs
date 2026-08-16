//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta820 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2933;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2934;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta820(t4132: f64, t5599: f64, t689: f64, t14103: f64, t9285: f64, t9674: f64, t13730: f64, t1420: f64, t2782: f64, t13726: f64, t9303: f64, t13725: f64, t1445: f64, t2439: f64, t14082: f64, t3920: f64, t14078: f64, t2470: f64, t3915: f64, t13735: f64, t2435: f64, t10119: f64, t14114: f64, t10115: f64, t1900: f64, t14189: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47929, t47932, t47936, t47938, t47942) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2933(t4132, t5599, t689, t14103, t9285, t9674, t13730, t1420, t2782, t13726, t9303, t13725, t1445, t2439);
        let (t47944, t47947, t47952, t47957, t47961, t47963) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2934(t14082, t3920, t14078, t2470, t3915, t13735, t2435, t10119, t14114, t10115, t1900, t14189);
    (t47929, t47932, t47936, t47938, t47942, t47944, t47947, t47952, t47957, t47961, t47963)
}
