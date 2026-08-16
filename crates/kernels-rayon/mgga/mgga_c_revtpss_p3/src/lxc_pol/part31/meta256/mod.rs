//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta256 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1133;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1134;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1135;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1136;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1137;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta256(t1937: f64, t2322: f64, t4254: f64, t1310: f64, t1936: f64, t651: f64, t112: f64, t624: f64, t655: f64, t68: f64, t114: f64, t665: f64, t508: f64, t2007: f64, t670: f64, t30: f64, t775: f64, t1949: f64, t212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6990, t6992, t6993) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1133(t1937, t2322, t4254, t1310, t1936);
        let (t6995, t6997, t6998) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1134(t651, t6993, t112, t624, t655, t68);
        let t7002 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1135(t114, t665, t6998, t6997);
        let t7003 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1136(t508, t7002);
        let (t7005, t7007, t7010, t7014) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1137(t651, t7003, t2007, t670, t30, t775, t1949, t212);
    (t6990, t6992, t6993, t6995, t6997, t6998, t7002, t7003, t7005, t7007, t7010, t7014)
}
