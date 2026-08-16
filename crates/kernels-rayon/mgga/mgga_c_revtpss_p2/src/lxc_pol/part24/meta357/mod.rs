//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta357 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1225;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta357(t225: f64, t24042: f64, t385: f64, t1695: f64, t6350: f64, t11121: f64, t23964: f64, t996: f64, t24031: f64, t1082: f64, t23640: f64, t378: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t24044, t24047, t24048, t24061, t24068, t24075, t24078) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1225(t225, t24042, t385, t1695, t6350, t11121, t23964, t996, t24031, t1082, t23640, t378);
    (t24044, t24047, t24048, t24061, t24068, t24075, t24078)
}
