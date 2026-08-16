//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta527 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1900;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1901;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1902;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta527(t116: f64, t7724: f64, t114: f64, t1513: f64, t25823: f64, t665: f64, t25826: f64, t4287: f64, t6998: f64, t25822: f64, t25824: f64, t508: f64, t651: f64, t118: f64, t1519: f64, t25805: f64, t27145: f64, t27152: f64, t27156: f64, t27830: f64, t27834: f64, t27835: f64, t28022: f64, t28025: f64, t4254: f64, t4257: f64, t4293: f64, t4297: f64, t671: f64, t6985: f64, t7746: f64) -> (f64, f64, f64, f64, f64, f64) {
        let t28030 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1900(t116, t7724);
        let (t28034, t28036, t28042) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1901(t114, t1513, t25823, t665, t25826, t4287, t6998, t25822, t25824);
        let (t28043, t28046) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1902(t28042, t508, t651, t118, t1519, t25805, t27145, t27152, t27156, t27830, t27834, t27835, t28022, t28025, t28030, t4254, t4257, t4293, t4297, t671, t6985, t7746);
    (t28030, t28034, t28036, t28042, t28043, t28046)
}
