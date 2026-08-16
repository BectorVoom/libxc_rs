//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 965/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk965(t12: f64, t24: f64, t10513: f64, t10518: f64, t1064: f64, t1837: f64, t207: f64, t3366: f64, t10523: f64, t10528: f64, t1165: f64, t2179: f64, t333: f64, t3374: f64, zeta_threshold: f64) -> (f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t10546 = piecewise3(t84, 0.0_f64, 8.0_f64 / 27.0_f64 * t1837 * t10513 - 2.0_f64 / 3.0_f64 * t1064 * t3366 + 2.0_f64 / 3.0_f64 * t207 * t10518);
    let t10554 = piecewise3(t90, 0.0_f64, 8.0_f64 / 27.0_f64 * t2179 * t10523 - 2.0_f64 / 3.0_f64 * t1165 * t3374 + 2.0_f64 / 3.0_f64 * t333 * t10528);
    (t10546, t10554)
}
