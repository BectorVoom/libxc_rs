//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta763 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2845;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta763(t1086: f64, t3259: f64, t994: f64, t3046: f64, t4980: f64, t12153: f64, t12046: f64, t989: f64, t1035: f64, t42859: f64, t342: f64, t12166: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t43357, t43360, t43378, t43384, t43400, t43401, t43420) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2845(t1086, t3259, t994, t3046, t4980, t12153, t12046, t989, t1035, t42859, t342, t12166);
    (t43357, t43360, t43378, t43384, t43400, t43401, t43420)
}
