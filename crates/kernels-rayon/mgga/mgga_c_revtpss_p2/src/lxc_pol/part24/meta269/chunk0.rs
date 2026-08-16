//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1041/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1041(t212: f64, t6041: f64, t780: f64, t689: f64, t2703: f64, t5985: f64, t10905: f64, t5989: f64, t5962: f64, t854: f64, t236: f64, t807: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18316 = t212 * t6041;
    let t18317 = t18316 * t780;
    let t18318 = t689 * t18317;
    let t18338 = t2703 * t5985;
    let t18340 = t10905 * t5989;
    let t18348 = t854 * t5962;
    let t18349 = t236 * t18348;
    let t18350 = t807 * t18349;
    (t18316, t18317, t18318, t18338, t18340, t18348, t18349, t18350)
}
