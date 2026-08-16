//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2212;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta489(t1065: f64, t4772: f64, t906: f64, t1042: f64, t2858: f64, t4823: f64, t1469: f64, t3059: f64, t4872: f64, t999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16138, t16139, t16140, t16143, t16144, t16147, t16148, t16149, t16152) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2212(t1065, t4772, t906, t1042, t2858, t4823, t1469, t3059, t4872, t999);
    (t16138, t16139, t16140, t16143, t16144, t16147, t16148, t16149, t16152)
}
