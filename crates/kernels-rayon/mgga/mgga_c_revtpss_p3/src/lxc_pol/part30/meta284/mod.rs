//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta284 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1246;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1247;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1248;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta284(t3: f64, t8240: f64, t1918: f64, t2170: f64, t573: f64, t7949: f64, t7952: f64, t7955: f64, t2033: f64, t4147: f64, param_d: f64, t587: f64, t65: f64, t3140: f64, t3736: f64, t1276: f64, t1243: f64, t197: f64, t532: f64, t1450: f64, t143: f64, t2580: f64, t130: f64, t2566: f64, t700: f64, t2584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t8241, t8245, t8249, t8717) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1246(t3, t8240, t1918, t2170, t573, t7949, t7952, t7955, t2033, t4147, param_d);
        let (t8779, t8939, t8945) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1247(t587, t65, t3140, t3736, t1276, t1243);
        let (t8995, t8996, t9275, t9278) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1248(t197, t532, t1450, t2033, t143, t2580, t130, t2566, t700, t2584);
    (t8241, t8245, t8249, t8717, t8779, t8939, t8945, t8995, t8996, t9275, t9278)
}
