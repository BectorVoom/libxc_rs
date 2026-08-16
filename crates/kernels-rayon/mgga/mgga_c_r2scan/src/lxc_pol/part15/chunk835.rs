//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 835/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk835(t1567: f64, t910: f64, t1570: f64, t2124: f64, t2562: f64, t360: f64, t2719: f64, t481: f64, t551: f64, t552: f64, t1632: f64, t2634: f64) -> (f64, f64, f64, f64, f64) {
    let t7533 = t1567 * t910;
    let t7535 = t2124 * t7533 * t1570;
    let t7538 = t2562 * t1570;
    let t7539 = t360 * t7538;
    let t7542 = t2719 * t481;
    let t7544 = t551 * t552 * t7542;
    let t7551 = t551 * t1632 * t2634;
    (t7535, t7538, t7539, t7544, t7551)
}
