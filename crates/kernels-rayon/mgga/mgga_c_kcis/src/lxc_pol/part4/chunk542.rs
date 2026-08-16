//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 542/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk542(t169: f64, t180: f64, t2791: f64, t980: f64, t442: f64, t911: f64, t916: f64, t1296: f64, t2635: f64, t234: f64, t441: f64, t233: f64, t1295: f64, t915: f64, sigma0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t170 = t169 <= zeta_threshold;
    let t2792 = t180 * t2791;
    let t2793 = 1.0_f64 / t980;
    let t2794 = sigma0 * t2793;
    let t2795 = t2794 * t442;
    let t2796 = t2795 / 8.0_f64;
    let t2797 = t911 * t916;
    let t2798 = t2797 / 8.0_f64;
    let t2799 = t911 * t1296;
    let t2800 = t2799 / 8.0_f64;
    let t2801 = piecewise3(t170, 0.0_f64, t2635);
    let t2802 = t234 * t2801;
    let t2803 = t2802 * t441;
    let t2804 = t233 * t2803;
    let t2805 = t2804 / 16.0_f64;
    let t2806 = t915 * t1295;
    (t2792, t2794, t2796, t2798, t2800, t2802, t2805, t2806)
}
