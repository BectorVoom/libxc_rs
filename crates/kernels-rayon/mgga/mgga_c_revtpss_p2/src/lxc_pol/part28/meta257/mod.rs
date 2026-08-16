//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1145;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1146;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1147;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta257(t651: f64, t6993: f64, t112: f64, t624: f64, t655: f64, t68: f64, t114: f64, t665: f64, t508: f64, t2007: f64, t670: f64, t30: f64, t775: f64, t1949: f64, t212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6995, t6997, t6998) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1145(t651, t6993, t112, t624, t655, t68);
        let t7002 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1146(t114, t665, t6998, t6997);
        let t7003 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1147(t508, t7002);
        let (t7005, t7007, t7010, t7014) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1148(t651, t7003, t2007, t670, t30, t775, t1949, t212);
    (t6995, t6997, t6998, t7002, t7003, t7005, t7007, t7010, t7014)
}
