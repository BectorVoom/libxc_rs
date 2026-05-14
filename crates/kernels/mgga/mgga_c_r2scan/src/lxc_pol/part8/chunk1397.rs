//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1397/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1397<F: Float>(t2207: F, t2691: F, t9268: F, t2201: F, t2837: F, t9434: F, t1592: F, t20426: F, t20445: F, t20471: F, t20484: F, t25399: F, t25406: F, t2614: F, t2636: F, t2719: F, t29733: F, t29742: F, t3016: F, t30853: F, t31037: F, t551: F, t552: F) -> (F,) {
    let t33884 = t2207 * t9268 * t2691;
    let t33887 = t2201 * t2837 * t9434;
    let t33903 = 0.29272321618148349056e-1 * t29733 - 0.52396431978519890152e-1 * t33884 - 0.17465477326173296717e-1 * t33887 - 0.25705033881751801528e-4 * t20426 - 0.34930954652346593433e-1 * t29742 + t20445 - 0.17888640988868435535e-2 * t20471 + 0.19776387377308997907e1 * t20484 - 0.12459097221822660494e0 * t25399 + 0.2600466522016280569e0 * t30853 * t2636 + 0.39006997830244208535e0 * t31037 * t2614 + 0.39006997830244208535e0 * t1592 * t551 * t552 * t2719 * t3016 - t25406;
    (t33903,)
}
