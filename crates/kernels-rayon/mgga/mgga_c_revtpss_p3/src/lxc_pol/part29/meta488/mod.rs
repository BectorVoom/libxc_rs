//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1772;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1773;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta488(t1450: f64, t5591: f64, t2013: f64, t8995: f64, t1448: f64, t1907: f64, t4292: f64, t93: f64, t2106: f64, t9593: f64, t198: f64, t205: f64, t2070: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t28176, t28196, t28198, t28219, t28286) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1772(t1450, t5591, t2013, t8995, t1448, t1907, t4292, t93, t2106, t9593);
        let (t28287, t28291) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1773(t28198, t28286, t198, t205, t2070);
    (t28176, t28196, t28198, t28219, t28286, t28287, t28291)
}
