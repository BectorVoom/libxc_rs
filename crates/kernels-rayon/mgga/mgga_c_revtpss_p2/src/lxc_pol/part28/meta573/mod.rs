//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2036;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta573(t25569: f64, t3111: f64, t11722: f64, t7132: f64, t11727: f64, t12002: f64, t1971: f64, t351: f64, t1052: f64, t3089: f64, t1087: f64, t11744: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93579, t93583, t93585, t93592, t93595, t93596, t93597, t93602) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2036(t25569, t3111, t11722, t7132, t11727, t12002, t1971, t351, t1052, t3089, t1087, t11744, sigma0);
    (t93579, t93583, t93585, t93592, t93595, t93596, t93597, t93602)
}
