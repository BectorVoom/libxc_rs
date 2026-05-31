//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1079/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1079<F: Float>(t1240: F, t3279: F, t3985: F, t550: F, t136: F, t3990: F, t680: F, t3946: F, t1319: F, t2950: F, t10446: F, t10450: F, t10457: F, t10461: F, t10463: F, t216: F, t2949: F, t3274: F, t3288: F, t3986: F, t677: F, t766: F) -> (F, F, F, F) {
    let t10465 = t1240 * t3279;
    let t10467 = t550 * t3985;
    let t10468 = t136 * t10467;
    let t10470 = t3990 * t680;
    let t10472 = t550 * t3946;
    let t10473 = t136 * t10472;
    let t10477 = t2950 * t1319;
    let t10480 = -F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t677 * t3986 - F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t136 * t10446 - F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t10450 * t216 - F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t3990 * t766 - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t1240 * t3274 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t2949 * t10457 - t10461 / F::cast_from(64.0_f64) - t10463 / F::cast_from(32.0_f64) - t10465 / F::cast_from(32.0_f64) - t10468 / F::cast_from(64.0_f64) - t10470 / F::cast_from(64.0_f64) - t10473 / F::cast_from(32.0_f64) - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t1240 * t3288 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t2949 * t10477;
    (t10467, t10472, t10477, t10480)
}
