//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1274/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1274(t3901: f64, t5573: f64, t1334: f64, t3899: f64, t3893: f64, t5577: f64, t11516: f64, t1906: f64, t3862: f64, t11513: f64, t1907: f64, t12780: f64, t5618: f64) -> (f64, f64, f64, f64, f64) {
    let t16263 = t5573 * t3901;
    let t16264 = t16263 * t1334;
    let t16266 = 0.32163648644302209644e2_f64 * t3899 * t16264;
    let t16267 = t5577 * t3893;
    let t16269 = 0.16081824322151104822e2_f64 * t3899 * t16267;
    let t16270 = t1906 * t11516;
    let t16271 = t16270 * t3862;
    let t16273 = 0.51725014705706168417e3_f64 * t11513 * t16271;
    let t16274 = t1907 * t3862;
    let t16276 = 6.0_f64 * t3899 * t16274;
    let t16277 = t5618 * t12780;
    (t16266, t16269, t16273, t16276, t16277)
}
