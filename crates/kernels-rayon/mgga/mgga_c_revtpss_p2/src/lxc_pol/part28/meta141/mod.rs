//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta141 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk773;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk774;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk775;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk776;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk777;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk778;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta141(t1063: f64, t3111: f64, t1086: f64, t994: f64, t3090: f64, t373: f64, t66: f64, t828: f64, t1043: f64, t999: f64, t1045: f64, t1032: f64, t989: f64, t1040: f64, t1024: f64, t1062: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3112, t3114) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk773(t1063, t3111, t1086, t994);
        let t3115 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk774(t3090, t3114);
        let t3116 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk775(t373, t66);
        let t3117 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk776(t3116, t828);
        let (t3118, t3119, t3120, t3123, t3124) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk777(t1043, t999, t1045, t3117, t1032, t989, t1040);
        let t3127 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk778(t1024, t1062);
    (t3112, t3114, t3115, t3116, t3117, t3118, t3119, t3120, t3123, t3124, t3127)
}
