//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1998;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta549(t1448: f64, t5591: f64, t1868: f64, t4144: f64, t1353: f64, t5778: f64, t1501: f64, t2371: f64, t4245: f64, t670: f64, t2037: f64, t4168: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t73394, t73488, t75353, t75365, t75485, t75667, t92556) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1998(t1448, t5591, t1868, t4144, t1353, t5778, t1501, t2371, t4245, t670, t2037, t4168);
    (t73394, t73488, t75353, t75365, t75485, t75667, t92556)
}
