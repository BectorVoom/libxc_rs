//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta397(t2470: f64, t4480: f64, t2465: f64, t11008: f64, t1579: f64, t2771: f64, t1558: f64, t836: f64, t231: f64, t2797: f64, t2782: f64, t860: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t14485, t14486, t14489, t14494, t14495, t14498, t14502) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1490(t2470, t4480, t2465, t11008, t1579, t2771, t1558, t836, t231, t2797, t2782, t860);
    (t14485, t14486, t14489, t14494, t14495, t14498, t14502)
}
