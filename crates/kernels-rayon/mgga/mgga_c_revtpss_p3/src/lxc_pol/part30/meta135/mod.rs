//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta135 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk741;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk742;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta135(t2962: f64, t954: f64, t944: f64, t302: f64, t310: f64, t2944: f64, t2846: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t324: f64, t960: f64, t964: f64, t320: f64, t963: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2963, t2966, t2967, t2968, t2969, t2970, t2971, t2974, t2979, t2980) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk741(t2962, t954, t944, t302, t310, t2944, t2846, t2848, t2855, t2860, t2864, t324);
        let (t2982, t2986) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk742(t960, t964, t320, t963);
    (t2963, t2966, t2967, t2968, t2969, t2970, t2971, t2974, t2979, t2980, t2982, t2986)
}
