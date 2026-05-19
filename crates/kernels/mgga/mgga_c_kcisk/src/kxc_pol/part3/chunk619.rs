//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 619/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk619<F: Float>(t4636: F, t4638: F, t4642: F, t4646: F, t4650: F, t1961: F, t1965: F, t1964: F, t760: F, t755: F, t1973: F, t1974: F) -> (F, F, F, F, F, F) {
    let t5360 = F::cast_from(0.22831111111111111111e-1_f64) * t4636;
    let t5365 = t5360 + F::cast_from(0.11415555555555555555e-1_f64) * t4638 - F::cast_from(0.11415555555555555555e-1_f64) * t4642 + F::cast_from(0.34246666666666666666e-1_f64) * t4646 - F::cast_from(0.17123333333333333333e-1_f64) * t4650;
    let t5368 = t1961 * t1965;
    let t5371 = t1964 * t760;
    let t5372 = F::new(1.0) / t5371;
    let t5373 = t755 * t5372;
    let t5374 = t1973 * t1973;
    let t5375 = t5374 * t1974;
    (t5365, t5368, t5372, t5373, t5374, t5375)
}
