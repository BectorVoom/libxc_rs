//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1023/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1023(t8344: f64, t921: f64, t2381: f64, t1167: f64, t2368: f64, t2371: f64, t2396: f64, t1235: f64, t2023: f64, t46: f64, t2394: f64, t1229: f64, t5939: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8345 = t8344 * t921;
    let t8346 = t2381 * t8345;
    let t8349 = t1167 * t2368;
    let t8350 = t8349 * t2371;
    let t8351 = t2381 * t8350;
    let t8354 = t8349 * t2396;
    let t8355 = t2381 * t8354;
    let t8358 = t1235 * t2023;
    let t8359 = t8358 * t46;
    let t8360 = t2394 * t8359;
    let t8363 = t5939 * t1229;
    (t8345, t8346, t8350, t8351, t8354, t8355, t8358, t8359, t8360, t8363)
}
