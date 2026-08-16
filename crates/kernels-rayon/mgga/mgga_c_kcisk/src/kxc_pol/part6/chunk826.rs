//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 826/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk826(t1305: f64, t8022: f64, t25: f64, t8055: f64, t1309: f64, t6157: f64, t6196: f64, t8049: f64, t425: f64, t7764: f64, t3521: f64, t7858: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26430 = t8022 * t1305;
    let t26470 = t25 * t8055;
    let t26471 = t1309 * t26470;
    let t26485 = t6157 * t6196;
    let t26489 = t25 * t8049;
    let t26490 = t1309 * t26489;
    let t26572 = t425 * t7764;
    let t26577 = t3521 * t7858;
    (t26430, t26471, t26485, t26490, t26572, t26577)
}
