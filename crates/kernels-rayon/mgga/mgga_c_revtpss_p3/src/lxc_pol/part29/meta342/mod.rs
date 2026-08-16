//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta342 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1262;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1263;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta342(t3111: f64, t3188: f64, t3211: f64, t3215: f64, t1026: f64, t371: f64, t676: f64, t1025: f64, t271: f64, t2857: f64, t283: f64, t3298: f64, t994: f64, t4891: f64, t1086: f64, t3046: f64, t3090: f64, t3316: f64, t1016: f64, t697: f64, t1011: f64, t1010: f64, t2270: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11802, t11814, t11818, t11821, t11852, t11858) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1262(t3111, t3188, t3211, t3215, t1026, t371, t676, t1025, t271, t2857, t283, t3298, t994);
        let (t11859, t11866, t11875, t11881, t11883) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1263(t11858, t4891, t1086, t3046, t3090, t3316, t994, t1016, t697, t1011, t1010, t2270);
    (t11802, t11814, t11818, t11821, t11852, t11859, t11866, t11875, t11881, t11883)
}
