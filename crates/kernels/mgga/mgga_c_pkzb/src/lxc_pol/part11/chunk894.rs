//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 894/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk894<F: Float>(t10481: F, t195: F, t1062: F, t3359: F, t3507: F, t998: F, t8718: F, t6804: F, t6811: F, t6819: F, t6821: F, t6823: F, t6826: F, t7012: F, t4867: F, t4870: F, t4876: F, t4879: F, t4881: F, t4884: F, t5077: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10482 = t10481 * t195;
    let t10483 = t3359 * t1062;
    let t10484 = 3.0 * t10483;
    let t10485 = t998 * t3507;
    let t10486 = 3.0 * t10485;
    let t10487 = 0.54934341918019635162e-3 * t8718;
    let t10488 = 0.73245789224026180216e-3 * t6804;
    let t10489 = 24.0 * t6811;
    let t10490 = 60.0 * t6819;
    let t10491 = 36.0 * t6821;
    let t10492 = 96.0 * t6823;
    let t10493 = 3.0 * t6826;
    let t10494 = 24.0 * t7012;
    let t10495 = t4867 + t4870 - t4876 - t4879 - t10487 + t10488 - t4881 - t10489 - t4884 + t10490 + t10491 + t10492 + t10493 + t5077 - t10494;
    (t10482, t10484, t10486, t10487, t10488, t10489, t10490, t10491, t10492, t10493, t10494, t10495)
}
