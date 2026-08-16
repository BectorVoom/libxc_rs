//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta518 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1937;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1938;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta518(t1096: f64, t7817: f64, t7160: f64, t7821: f64, t988: f64, t7145: f64, t1035: f64, t7810: f64, t1043: f64, t1089: f64, t1982: f64, t27418: f64, t342: f64, t1678: f64, t3140: f64, t1078: f64, t1668: f64, t25681: f64, t4866: f64, t7168: f64, t7828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27594, t27595, t27598, t27599, t27604, t27606, t27609) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1937(t1096, t7817, t7160, t7821, t988, t7145, t1035, t7810, t1043, t1089, t1982, t27418);
        let (t27616, t27621, t27627, t27631, t27634) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1938(t342, t7810, t1678, t3140, t1078, t1982, t1089, t1668, t25681, t4866, t7168, t7828, t988);
    (t27594, t27595, t27598, t27599, t27604, t27606, t27609, t27616, t27621, t27627, t27631, t27634)
}
