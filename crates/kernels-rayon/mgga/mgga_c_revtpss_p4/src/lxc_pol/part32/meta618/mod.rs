//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta618 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1958;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta618(t22267: f64, t25997: f64, t22255: f64, t7264: f64, t22259: f64, t22276: f64, t7271: f64, t22281: f64, t26024: f64, t6876: f64, t22289: f64, t22115: f64, t26028: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t108566, t108568, t108570, t108572, t108574, t108576, t108578, t108583) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1958(t22267, t25997, t22255, t7264, t22259, t22276, t7271, t22281, t26024, t6876, t22289, t22115, t26028);
    (t108566, t108568, t108570, t108572, t108574, t108576, t108578, t108583)
}
