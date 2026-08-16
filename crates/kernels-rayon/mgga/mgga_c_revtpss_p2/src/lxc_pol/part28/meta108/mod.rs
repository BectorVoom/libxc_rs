//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta108 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk663;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk664;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk665;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk666;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta108(t221: f64, t2485: f64, t837: f64, t2484: f64, t737: f64, t744: f64, t185: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2487, t2488, t2490, t2491) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk663(t221, t2485, t837, t2484, t737);
        let t2492 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk664(t744);
        let (t2494, t2495) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk665(t185);
        let t2496 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk666(t2491, t2492, t2495);
    (t2487, t2488, t2490, t2491, t2492, t2494, t2495, t2496)
}
