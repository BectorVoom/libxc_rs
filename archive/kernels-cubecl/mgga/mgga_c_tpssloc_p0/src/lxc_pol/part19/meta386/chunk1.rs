//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1448/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1448<F: Float>(t3450: F, t9258: F, t11571: F, t11584: F, t11593: F, t1174: F, t24705: F, t3447: F, t3449: F, t3451: F, t3469: F, t43719: F, t43723: F, t44499: F, t44502: F, t44504: F, t44506: F, t44512: F, t44517: F, t44521: F, t44527: F, t44529: F, t44536: F, t4908: F, t4934: F) -> F {
    let t44540 = t3450 * t9258;
    let t44547 = -F::cast_from(0.49999999999999999999e-2_f64) * t1174 * t4934 * t24705 * t3469 + F::cast_from(0.29629629629629629628e-2_f64) * t44499 - F::cast_from(0.22222222222222222221e-2_f64) * t44502 + F::cast_from(0.34567901234567901234e-2_f64) * t3447 * t44504 * t44506 + F::cast_from(0.11111111111111111111e-2_f64) * t44512 + F::cast_from(0.33333333333333333332e-2_f64) * t3447 * t11593 * t11584 + F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t44517 * t3451 + F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t44521 * t3451 - F::cast_from(0.14814814814814814814e-2_f64) * t44527 - F::cast_from(0.22222222222222222222e-2_f64) * t3447 * t44529 * t11571 - F::cast_from(0.99999999999999999996e-2_f64) * t3447 * t4908 * t43719 + F::cast_from(0.66666666666666666664e-2_f64) * t3447 * t3449 * t44536 + F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t3449 * t44540 - F::cast_from(0.22222222222222222221e-2_f64) * t3447 * t4908 * t43723;
    t44547
}
