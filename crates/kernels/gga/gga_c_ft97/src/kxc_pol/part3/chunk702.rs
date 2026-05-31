//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 702/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk702<F: Float>(t1882: F, t3480: F, t3485: F, t1045: F, t2178: F, t3584: F, t3580: F, t3571: F, t3442: F, t8392: F, t582: F, t167: F, t9132: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12642 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t3480;
    let t12644 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1882 * t3485;
    let t12664 = t1045 * t2178;
    let t12670 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t3584;
    let t12672 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t3580;
    let t12674 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t3571;
    let t12676 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t8392 * t3442;
    let t12680 = t582 * t1045;
    let t12703 = t9132 * t167;
    (t12642, t12644, t12664, t12670, t12672, t12674, t12676, t12680, t12703)
}
