//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2035;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta581(t94564: f64, t9795: f64, t2018: f64, t40688: f64, t46808: f64, t7256: f64, t9784: f64, t1445: f64, t2439: f64, t25916: f64, t1358: f64, t212: f64, t26034: f64, t689: f64) -> (f64, f64, f64, f64, f64) {
        let (t94565, t94569, t94571, t94580, t94584) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2035(t94564, t9795, t2018, t40688, t46808, t7256, t9784, t1445, t2439, t25916, t1358, t212, t26034, t689);
    (t94565, t94569, t94571, t94580, t94584)
}
