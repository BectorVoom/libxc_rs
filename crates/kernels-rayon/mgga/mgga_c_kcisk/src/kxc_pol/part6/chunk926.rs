//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 926/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk926(t1919: f64, t2063: f64, t24434: f64, t28368: f64, t5249: f64, t7389: f64, t7718: f64, t1920: f64, t28312: f64, t11832: f64, t5248: f64, t17991: f64, t7715: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29441 = t1919 * t24434 * t2063;
    let t29445 = t1919 * t5249 * t28368;
    let t29449 = t1919 * t7389 * t7718;
    let t29453 = t1919 * t1920 * t28312;
    let t29462 = t5248 * t11832 * t28368;
    let t29466 = t1919 * t17991 * t7715;
    (t29441, t29445, t29449, t29453, t29462, t29466)
}
