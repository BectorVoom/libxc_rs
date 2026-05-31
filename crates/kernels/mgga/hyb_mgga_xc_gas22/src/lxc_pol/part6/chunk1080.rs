//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1080/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1080<F: Float>(t3: F, t3160: F, t1238: F, t8561: F, t3282: F, t8492: F, t2033: F, t3938: F, t10325: F, t688: F, t3150: F, t4089: F, t6227: F, t6468: F, t6471: F, t6485: F, t677: F, t684: F, t687: F, t8560: F, t8575: F, t8577: F, t8579: F) -> (F, F, F, F, F, F, F) {
    let t10486 = t3160 * t3;
    let t10490 = t8561 * t1238;
    let t10494 = t3282 * t3;
    let t10498 = t8492 * t1238;
    let t10505 = t2033 * t3938;
    let t10509 = t688 * t10325;
    let t10513 = -F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t677 * t4089 - t6468 + t6485 / F::cast_from(96.0_f64) + t6227 / F::cast_from(96.0_f64) + t6471 / F::cast_from(288.0_f64) + t684 * t3150 * t10486 / F::cast_from(16.0_f64) - t684 * t687 * t10490 / F::cast_from(32.0_f64) + t684 * t3150 * t10494 / F::cast_from(16.0_f64) - t8560 - t684 * t687 * t10498 / F::cast_from(32.0_f64) + t8575 / F::cast_from(144.0_f64) + t8577 / F::cast_from(48.0_f64) + t8579 / F::cast_from(16.0_f64) - t684 * t687 * t10505 / F::cast_from(64.0_f64) - t684 * t687 * t10509 / F::cast_from(64.0_f64);
    (t10486, t10490, t10494, t10498, t10505, t10509, t10513)
}
