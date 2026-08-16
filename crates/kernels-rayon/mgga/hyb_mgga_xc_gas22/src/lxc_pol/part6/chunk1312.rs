//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1312/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1312(t10176: f64, t2024: f64, t6479: f64, t3939: f64, t6469: f64, t684: f64, t3930: f64, t1240: f64, t8545: f64, t10164: f64, t2014: f64, t10434: f64, t10438: f64, t10446: f64, t10457: f64, t10477: f64, t1318: f64, t136: f64, t2004: f64, t2026: f64, t2152: f64, t24461: f64, t24464: f64, t24468: f64, t26: f64, t27770: f64, t2949: f64, t2950: f64, t3273: f64, t3984: f64, t3986: f64, t4089: f64, t457: f64, t677: f64, t688: f64, t7831: f64, t8439: f64) -> f64 {
    let t28647 = t2024 * t6479 * t10176;
    let t28655 = t684 * t6469 * t3939;
    let t28658 = t684 * t6469 * t3930;
    let t28678 = t1240 * t8545;
    let t28691 = t684 * t2014 * t10164;
    let t28693 = -t28647 / 72.0_f64 - t2024 * t27770 * t2026 * t688 * t457 / 6.0_f64 + t28655 / 288.0_f64 + t28658 / 144.0_f64 - 3.0_f64 / 32.0_f64 * t677 * t10434 - 3.0_f64 / 64.0_f64 * t2004 * t4089 - 3.0_f64 / 64.0_f64 * t136 * t26 * t2152 * t3984 - 3.0_f64 / 32.0_f64 * t677 * t10446 - 3.0_f64 / 32.0_f64 * t136 * t26 * t8439 * t1318 - 3.0_f64 / 64.0_f64 * t2004 * t3986 - 3.0_f64 / 16.0_f64 * t677 * t10438 - t28678 / 16.0_f64 + 3.0_f64 / 16.0_f64 * t7831 * t10477 + 3.0_f64 / 16.0_f64 * t7831 * t10457 + 3.0_f64 / 8.0_f64 * t2949 * t2950 * t3273 - t24461 / 48.0_f64 + t24464 / 72.0_f64 + 41.0_f64 / 144.0_f64 * t24468 - t28691 / 48.0_f64;
    t28693
}
