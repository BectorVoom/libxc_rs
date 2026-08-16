//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 422/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk422(t206: f64, t2726: f64, t2728: f64, t20: f64, t2394: f64, t62: f64, t212: f64, t879: f64, t882: f64, t209: f64, t2718: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t210 = 0.0_f64 < t206;
    let t2729 = t2726 * t2728;
    let t2733 = t62 * t2394 * t20;
    let t2739 = 1.0_f64 / t879 / t212;
    let t2740 = t882 * t882;
    let t2742 = t209 * t2739 * t2740;
    let t2746 = piecewise3(t210, t2718, -t2718);
    (t2729, t2733, t2739, t2740, t2742, t2746)
}
