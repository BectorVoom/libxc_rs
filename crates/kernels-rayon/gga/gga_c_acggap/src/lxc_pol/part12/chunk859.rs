//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 859/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk859(t30049: f64, t7548: f64, t137: f64, t3101: f64, t1089: f64, t1095: f64, t2079: f64, t19: f64, t3220: f64, t336: f64, t3116: f64, t368: f64) -> (f64, f64, f64, f64, f64) {
    let t30050 = t30049 * t7548;
    let t30052 = t137 * t3101;
    let t30055 = t2079 * t1089 * t1095 * t30052;
    let t30058 = t3220 * t19 * t336;
    let t30059 = t368 * t3116;
    (t30050, t30052, t30055, t30058, t30059)
}
