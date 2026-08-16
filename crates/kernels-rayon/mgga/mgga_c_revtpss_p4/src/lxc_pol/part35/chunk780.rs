//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 780/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk780(t12295: f64, t3566: f64, t3754: f64, t1209: f64, t5462: f64, t5477: f64, t3634: f64, t828: f64, t3618: f64, t3781: f64, t5330: f64, t1121: f64, t3603: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12678 = 0.25925925925925925926e-1_f64 * t12295;
    let t12717 = t3566 * t3754;
    let t12751 = t1209 * t5462;
    let t12756 = t1209 * t5477;
    let t12772 = t828 * t3634;
    let t12787 = t828 * t3618;
    let t12808 = t1209 * t3781;
    let t12809 = t12808 * t5330;
    let t12839 = t3603 * t1121;
    (t12678, t12717, t12751, t12756, t12772, t12787, t12809, t12839)
}
