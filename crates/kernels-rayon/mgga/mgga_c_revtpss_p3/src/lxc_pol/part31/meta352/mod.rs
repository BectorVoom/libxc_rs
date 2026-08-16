//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta352 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1366;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta352(t3930: f64, t5661: f64, t5665: f64, t9976: f64, t1412: f64, t1882: f64, t3938: f64, t3992: f64, t2661: f64, t1399: f64, t5608: f64, t5651: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14042, t14043, t14045, t14046, t14049, t14050, t14053, t14054) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1366(t3930, t5661, t5665, t9976, t1412, t1882, t3938, t3992, t2661, t1399, t5608, t5651);
    (t14042, t14043, t14045, t14046, t14049, t14050, t14053, t14054)
}
