//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2042;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2043;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta612(t25904: f64, t97899: f64, t1358: f64, t212: f64, t27960: f64, t689: f64, t26050: f64, t27899: f64, t2453: f64, t27883: f64, t25946: f64, t27873: f64, t94890: f64, t136: f64, t2457: f64, t7929: f64, t25944: f64, t2470: f64, t27887: f64, t7284: f64, t1955: f64, t27836: f64, t4075: f64, t26072: f64, t27888: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97900, t97908, t97915, t97917, t97920) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2042(t25904, t97899, t1358, t212, t27960, t689, t26050, t27899, t2453, t27883, t25946, t27873, t94890);
        let (t97922, t97923, t97925, t97926, t97933, t97943) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2043(t136, t2457, t7929, t25944, t2470, t27887, t7284, t1955, t27836, t4075, t26072, t27888);
    (t97900, t97908, t97915, t97917, t97920, t97922, t97923, t97925, t97926, t97933, t97943)
}
