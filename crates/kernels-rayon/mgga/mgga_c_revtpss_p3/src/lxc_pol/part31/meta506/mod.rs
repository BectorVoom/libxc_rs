//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta506 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1832;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta506(t1955: f64, t4469: f64, t72: f64, t7778: f64, t686: f64, t7064: f64, t1558: f64, t231: f64, t7048: f64, t7076: f64, t1949: f64, t4423: f64, t1959: f64, t25297: f64, t25303: f64, t25307: f64, t25311: f64, t25333: f64, t25337: f64, t25340: f64, t25353: f64, t25356: f64, t25383: f64, t7070: f64, t7775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27275, t27278, t27279) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1832(t1955, t4469, t72, t7778, t686);
        let (t27280, t27286, t27287, t27291, t27292, t27297) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1833(t27279, t7064, t1558, t231, t7048, t7076, t1949, t4423, t1959, t25297, t25303, t25307, t25311, t25333, t25337, t25340, t25353, t25356, t25383, t27275, t7070, t7775);
    (t27275, t27278, t27279, t27280, t27286, t27287, t27291, t27292, t27297)
}
