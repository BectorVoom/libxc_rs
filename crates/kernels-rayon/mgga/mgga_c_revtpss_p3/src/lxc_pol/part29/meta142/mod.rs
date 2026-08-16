//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta142 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk737;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk738;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk739;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk740;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk741;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta142(t3109: f64, t906: f64, t247: f64, t1063: f64, t1086: f64, t994: f64, t3090: f64, t373: f64, t66: f64, t828: f64, t1043: f64, t999: f64, t1045: f64, t1032: f64, t989: f64, t1040: f64, t1024: f64, t1062: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3111, t3112, t3114, t3115) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk737(t3109, t906, t247, t1063, t1086, t994, t3090);
        let t3116 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk738(t373, t66);
        let t3117 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk739(t3116, t828);
        let (t3118, t3119, t3120, t3123, t3124) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk740(t1043, t999, t1045, t3117, t1032, t989, t1040);
        let t3127 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk741(t1024, t1062);
    (t3111, t3112, t3114, t3115, t3116, t3117, t3118, t3119, t3120, t3123, t3124, t3127)
}
