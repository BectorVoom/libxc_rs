//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1448/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1448(t3450: f64, t9258: f64, t11571: f64, t11584: f64, t11593: f64, t1174: f64, t24705: f64, t3447: f64, t3449: f64, t3451: f64, t3469: f64, t43719: f64, t43723: f64, t44499: f64, t44502: f64, t44504: f64, t44506: f64, t44512: f64, t44517: f64, t44521: f64, t44527: f64, t44529: f64, t44536: f64, t4908: f64, t4934: f64) -> f64 {
    let t44540 = t3450 * t9258;
    let t44547 = -0.49999999999999999999e-2_f64 * t1174 * t4934 * t24705 * t3469 + 0.29629629629629629628e-2_f64 * t44499 - 0.22222222222222222221e-2_f64 * t44502 + 0.34567901234567901234e-2_f64 * t3447 * t44504 * t44506 + 0.11111111111111111111e-2_f64 * t44512 + 0.33333333333333333332e-2_f64 * t3447 * t11593 * t11584 + 0.11111111111111111111e-2_f64 * t3447 * t44517 * t3451 + 0.11111111111111111111e-2_f64 * t3447 * t44521 * t3451 - 0.14814814814814814814e-2_f64 * t44527 - 0.22222222222222222222e-2_f64 * t3447 * t44529 * t11571 - 0.99999999999999999996e-2_f64 * t3447 * t4908 * t43719 + 0.66666666666666666664e-2_f64 * t3447 * t3449 * t44536 + 0.11111111111111111111e-2_f64 * t3447 * t3449 * t44540 - 0.22222222222222222221e-2_f64 * t3447 * t4908 * t43723;
    t44547
}
