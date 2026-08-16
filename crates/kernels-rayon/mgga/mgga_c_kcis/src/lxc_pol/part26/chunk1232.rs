//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1232/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1232(t7898: f64, t94491: f64, t3245: f64, t7932: f64, t7935: f64, t10470: f64, t2244: f64, t2237: f64, t2238: f64, t737: f64, t61287: f64, t7907: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94524 = t7898 * t94491;
    let t94537 = t3245 * t7932;
    let t94539 = t3245 * t7935;
    let t94588 = t10470 * t2244;
    let t94589 = 0.73697530864197530862e-3_f64 * t94588;
    let t94614 = 0.25742669753086419753e-3_f64 * t2237 * t737 * t2238;
    let t94626 = t7907 * t61287;
    (t94524, t94537, t94539, t94588, t94589, t94614, t94626)
}
