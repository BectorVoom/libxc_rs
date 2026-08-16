//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2871/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2871(t225: f64, t45384: f64, t12627: f64, t1269: f64, t3566: f64, t3727: f64, t12640: f64, t44842: f64, t487: f64, t44420: f64, t13180: f64, t493: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t45385 = t45384 * t225;
    let t45427 = t12627 * t1269;
    let t45430 = t3566 * t3727;
    let t45433 = t12640 * t1269;
    let t45438 = t44842 * t487;
    let t45449 = t45384 * t487;
    let t45482 = t44420 * t487;
    let t45551 = 1.0_f64 / t13180 / t493;
    (t45385, t45427, t45430, t45433, t45438, t45449, t45482, t45551)
}
