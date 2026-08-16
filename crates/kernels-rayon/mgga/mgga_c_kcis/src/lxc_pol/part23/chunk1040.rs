//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1040/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1040(t16937: f64, t7910: f64, t7908: f64, t2645: f64, t4163: f64, t7923: f64, t1394: f64, t2642: f64, t5662: f64, t4153: f64, t1467: f64, t491: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27376 = t16937 * t7910;
    let t27377 = t7908 * t27376;
    let t27379 = t4163 * t2645;
    let t27380 = t7923 * t27379;
    let t27381 = t1394 * t27380;
    let t27383 = t5662 * t2642;
    let t27384 = t7923 * t27383;
    let t27385 = t4153 * t27384;
    let t27387 = t1467 * t491;
    (t27376, t27377, t27379, t27380, t27381, t27383, t27384, t27385, t27387)
}
