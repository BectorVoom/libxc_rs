//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 652/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk652<F: Float>(t3641: F, t3661: F, t1278: F, t1281: F, t1280: F, t436: F, t433: F, t1291: F, t3335: F, t3340: F, t3344: F, t3349: F, t3356: F, t3359: F, t3363: F, t3366: F, t3370: F, t3426: F, t3430: F, t3433: F) -> (F, F, F, F, F, F) {
    let t3662 = t3641 + t3661;
    let t3664 = t1278 * t1281;
    let t3668 = F::new(1.0) / t1280 / t436;
    let t3669 = t433 * t3668;
    let t3670 = t1291 * t1291;
    let t3685 = F::new(0.5e0) * t3335 - F::new(0.125e0) * t3340 + F::new(0.625e-1) * t3344 - F::cast_from(0.44965277777777777777e-2_f64) * t3349 - F::cast_from(0.34173611111111111111e0_f64) * t3356 + F::cast_from(0.14388888888888888889e0_f64) * t3359 + F::cast_from(0.91666666666666666667e0_f64) * t3363 - F::cast_from(0.33333333333333333334e0_f64) * t3366 - F::cast_from(0.101171875e-1_f64) * t3370 + F::new(0.9375e-1) * t3426 - F::cast_from(0.20833333333333333333e-1_f64) * t3430 - F::cast_from(0.10791666666666666667e0_f64) * t3433;
    (t3662, t3664, t3668, t3669, t3670, t3685)
}
