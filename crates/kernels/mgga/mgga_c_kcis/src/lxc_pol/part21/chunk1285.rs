//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1285/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1285<F: Float>(t1003: F, t26685: F, t27772: F, t27773: F, t27812: F, t3040: F, t7703: F, t7706: F, t95537: F, t95629: F, t95636: F, t95640: F, t95645: F, t95649: F, t95653: F, t95658: F, t95662: F, t95666: F, t95670: F) -> F {
    let t95681 = -F::cast_from(0.13901041666666666667e-2_f64) * t7703 * t95636 - F::cast_from(0.46336805555555555556e-3_f64) * t95640 * t7706 - F::cast_from(0.13901041666666666667e-2_f64) * t7703 * t95645 - F::cast_from(0.69505208333333333333e-3_f64) * t7703 * t95649 - F::cast_from(0.33163888888888888888e-2_f64) * t95653 - F::new(0.1492375e-1) * t95658 - F::cast_from(0.2653111111111111111e-1_f64) * t95662 - F::cast_from(0.33163888888888888888e-2_f64) * t95666 + F::cast_from(0.111403033060546875e-3_f64) * t27812 * t95537 - F::cast_from(0.27802083333333333334e-2_f64) * t7703 * t27772 * t95670 * t1003 - F::cast_from(0.13901041666666666667e-2_f64) * t7703 * t27772 * t27773 * t3040 - F::cast_from(0.2782641015625e-3_f64) * t26685 * t95629;
    t95681
}
