//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 575/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk575<F: Float>(t385: F, t4143: F, t8010: F, t1284: F, t7831: F, t2147: F, t2153: F, t340: F, t379: F, t382: F, t8003: F, t395: F, t3953: F, t7706: F, sigma0: F) -> (F, F, F, F, F) {
    let t386 = t385 < -F::cast_from(0.66725e-1_f64);
    let t8011 = t4143 * t8010;
    let t8015 = t1284 * t7831;
    let t8020 = piecewise3::<F>(t386, F::cast_from(0.0_f64), F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t340 * t8003 * t382 - F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t340 * t2147 * t2153 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t340 * t379 * t8011 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t340 * t379 * t8015);
    let t8021 = t8020 * sigma0;
    let t8022 = t8021 * t395;
    let t8032 = t3953 * t7706;
    (t8011, t8015, t8021, t8022, t8032)
}
