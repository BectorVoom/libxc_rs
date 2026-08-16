//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta516 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1870;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1871;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta516(t27641: f64, t73: f64, t4975: f64, t988: f64, t4976: f64, t27418: f64, t994: f64, t1096: f64, t27638: f64, t3143: f64, t1983: f64, t27642: f64, t4983: f64, t1984: f64, t27543: f64, t359: f64, t1646: f64, t7135: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27651, t27652, t27653, t27656, t27661) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1870(t27641, t73, t4975, t988, t4976, t27418, t994);
        let (t27664, t27665, t27668, t27669, t27670, t27676, t27679) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1871(t1096, t4975, t27651, t27638, t3143, t1983, t27642, t4983, t1984, t27543, t359, t1646, t7135);
    (t27652, t27653, t27656, t27661, t27664, t27665, t27668, t27669, t27670, t27676, t27679)
}
