//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 662/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk662(t3053: f64, t450: f64, t1112: f64, t242: f64, t359: f64, t461: f64, t651: f64, t460: f64, t1127: f64, t126: f64) -> (f64, f64, f64, f64, f64) {
    let t3081 = t3053 * t450;
    let t3082 = t1112 * t3081;
    let t3083 = t242 * t3082;
    let t3087 = t359 * t651 * t461;
    let t3089 = t460 * t3087 / 13824.0_f64;
    let t3090 = t126 * t1127;
    (t3081, t3083, t3087, t3089, t3090)
}
