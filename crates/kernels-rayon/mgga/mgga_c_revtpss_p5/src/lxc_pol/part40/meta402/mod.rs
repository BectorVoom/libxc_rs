//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1478;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta402(t114: f64, t31142: f64, t8315: f64, t2366: f64, t8311: f64, t104: f64, t2357: f64, t2358: f64, t2362: f64, t31035: f64, t31134: f64, t31135: f64, t31137: f64, t31139: f64, t8258: f64, t8267: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t31143, t31146, t31149, t31150, t31153, t31157) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1478(t114, t31142, t8315, t2366, t8311, t104, t2357, t2358, t2362, t31035, t31134, t31135, t31137, t31139, t8258, t8267);
    (t31143, t31146, t31149, t31150, t31153, t31157)
}
