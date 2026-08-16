//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 663/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk663(t1015: f64, t3090: f64, t242: f64, t1125: f64, t1014: f64, t400: f64) -> (f64, f64, f64) {
    let t3091 = t3090 * t1015;
    let t3092 = t242 * t3091;
    let t3093 = t1125 * t3092;
    let t3096 = 1.0_f64 / t400 / t1014;
    (t3092, t3093, t3096)
}
