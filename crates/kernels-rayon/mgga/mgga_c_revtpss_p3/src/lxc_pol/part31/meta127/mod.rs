//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta127 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk705;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk706;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk707;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk708;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta127(t3116: f64, t828: f64, t1032: f64, t989: f64, t1040: f64, t1024: f64, t1062: f64, t1031: f64, t196: f64, t342: f64, t1034: f64, t358: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t3117 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk705(t3116, t828);
        let (t3123, t3124, t3127) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk706(t1032, t989, t1040, t1024, t1062);
        let t3140 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk707(t1031, t196);
        let t3141 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk708(t3140, t342);
        let t3143 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk709(t1034, t358);
    (t3117, t3123, t3124, t3127, t3140, t3141, t3143)
}
