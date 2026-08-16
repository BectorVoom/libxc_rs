//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1274/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1274<F: Float>(t3901: F, t5573: F, t1334: F, t3899: F, t3893: F, t5577: F, t11516: F, t1906: F, t3862: F, t11513: F, t1907: F, t12780: F, t5618: F) -> (F, F, F, F, F) {
    let t16263 = t5573 * t3901;
    let t16264 = t16263 * t1334;
    let t16266 = F::cast_from(0.32163648644302209644e2_f64) * t3899 * t16264;
    let t16267 = t5577 * t3893;
    let t16269 = F::cast_from(0.16081824322151104822e2_f64) * t3899 * t16267;
    let t16270 = t1906 * t11516;
    let t16271 = t16270 * t3862;
    let t16273 = F::cast_from(0.51725014705706168417e3_f64) * t11513 * t16271;
    let t16274 = t1907 * t3862;
    let t16276 = F::cast_from(6.0_f64) * t3899 * t16274;
    let t16277 = t5618 * t12780;
    (t16266, t16269, t16273, t16276, t16277)
}
