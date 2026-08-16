//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta81 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk515;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk516;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk517;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk518;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk519;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk520;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta81(t1610: f64, t915: f64, t1594: f64, t939: f64, t1601: f64, t1604: f64, t1607: f64, t948: f64, t951: f64, t954: f64, t958: f64, t324: f64, t967: f64, t970: f64, t973: f64, t1598: f64, t300: f64, t311: f64, t946: f64, t965: f64, t964: f64, t981: f64, t986: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1612, t1614, t1621, t1622) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk515(t1610, t915, t1594, t939, t1601, t1604, t1607, t948, t951, t954);
        let t1626 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk516(t1594, t958);
        let (t1627, t1633) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk517(t1626, t324, t1594, t1601, t1604, t1607, t967, t970);
        let t1634 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk518(t1633, t973);
        let (t1638, t1640, t1642) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk519(t1598, t1612, t1614, t1622, t1627, t1634, t300, t311, t946, t965, t1633, t964, t973);
        let (t1644, t1646) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk520(t1642, t981, t1594, t986);
    (t1612, t1614, t1621, t1622, t1626, t1633, t1634, t1638, t1640, t1642, t1644, t1646)
}
