//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta560 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1971;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta560(t560: f64, t9655: f64, t1389: f64, t268: f64, t10115: f64, t555: f64, t4146: f64, t198: f64, t775: f64, t11821: f64, t65: f64, t2246: f64, t4171: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t46361, t46808, t47567, t47672, t50080, t53321, t60221) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1971(t560, t9655, t1389, t268, t10115, t555, t4146, t198, t775, t11821, t65, t2246, t4171);
    (t46361, t46808, t47567, t47672, t50080, t53321, t60221)
}
