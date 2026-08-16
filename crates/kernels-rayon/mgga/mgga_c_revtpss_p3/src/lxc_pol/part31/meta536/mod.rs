//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta536 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta536(t27240: f64, t27246: f64, t27251: f64, t27254: f64, t27256: f64, t28034: f64, t27924: f64, t27926: f64, t27929: f64, t27937: f64, t27955: f64, t1450: f64, t6816: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28330, t28333, t28335, t28336, t28337, t28679, t28872, t28873, t28874, t28877, t28885, t29494) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1921(t27240, t27246, t27251, t27254, t27256, t28034, t27924, t27926, t27929, t27937, t27955, t1450, t6816);
    (t28330, t28333, t28335, t28336, t28337, t28679, t28872, t28873, t28874, t28877, t28885, t29494)
}
