//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1850;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1851;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta511(t1976: f64, t4742: f64, t7145: f64, t1695: f64, t7135: f64, t7160: f64, t1043: f64, t1089: f64, t7817: f64, t7821: f64, t1096: f64, t7810: f64, t988: f64, t4820: f64, t7122: f64, t4878: f64, t7121: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27422, t27423, t27426, t27427, t27433, t27437, t27440) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1850(t1976, t4742, t7145, t1695, t7135, t7160, t1043, t1089, t7817, t7821, t1096, t7810);
        let (t27441, t27445, t27448, t27450) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1851(t27440, t7160, t7810, t988, t7145, t4820, t7122, t4878, t7121);
    (t27422, t27423, t27426, t27427, t27433, t27437, t27441, t27445, t27448, t27450)
}
