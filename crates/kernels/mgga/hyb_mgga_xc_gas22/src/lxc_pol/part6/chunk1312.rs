//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1312/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1312<F: Float>(t10176: F, t2024: F, t6479: F, t3939: F, t6469: F, t684: F, t3930: F, t1240: F, t8545: F, t10164: F, t2014: F, t10434: F, t10438: F, t10446: F, t10457: F, t10477: F, t1318: F, t136: F, t2004: F, t2026: F, t2152: F, t24461: F, t24464: F, t24468: F, t26: F, t27770: F, t2949: F, t2950: F, t3273: F, t3984: F, t3986: F, t4089: F, t457: F, t677: F, t688: F, t7831: F, t8439: F) -> F {
    let t28647 = t2024 * t6479 * t10176;
    let t28655 = t684 * t6469 * t3939;
    let t28658 = t684 * t6469 * t3930;
    let t28678 = t1240 * t8545;
    let t28691 = t684 * t2014 * t10164;
    let t28693 = -t28647 / F::cast_from(72.0_f64) - t2024 * t27770 * t2026 * t688 * t457 / F::cast_from(6.0_f64) + t28655 / F::cast_from(288.0_f64) + t28658 / F::cast_from(144.0_f64) - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t677 * t10434 - F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t2004 * t4089 - F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t136 * t26 * t2152 * t3984 - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t677 * t10446 - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t136 * t26 * t8439 * t1318 - F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t2004 * t3986 - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t677 * t10438 - t28678 / F::cast_from(16.0_f64) + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t7831 * t10477 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t7831 * t10457 + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t2949 * t2950 * t3273 - t24461 / F::cast_from(48.0_f64) + t24464 / F::cast_from(72.0_f64) + F::cast_from(41.0_f64) / F::cast_from(144.0_f64) * t24468 - t28691 / F::cast_from(48.0_f64);
    t28693
}
