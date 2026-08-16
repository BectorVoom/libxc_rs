//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 644/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk644<F: Float>(t322: F, t1048: F, t3719: F, t499: F, t2867: F, t3275: F, t3465: F, t3500: F, t3504: F, t3625: F, t3627: F, t3630: F) -> (F, F, F, F) {
    let t324 = F::cast_from(0.0_f64) < t322;
    let t3721 = t1048 * t499 * t3719;
    let t3722 = t3721 / F::cast_from(4.0_f64);
    let t3724 = t3275 * t3465 * t2867;
    let t3725 = t3724 / F::cast_from(4.0_f64);
    let t3729 = t3500 + t3625 / F::cast_from(4.0_f64) - t3627 / F::cast_from(4.0_f64) + t3630 / F::cast_from(2.0_f64) + t3504;
    let t3730 = piecewise3::<F>(t324, F::cast_from(0.0_f64), t3729);
    (t3722, t3725, t3729, t3730)
}
