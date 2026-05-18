//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 968/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk968<F: Float>(t18961: F, t3691: F, t2917: F, t294: F, t3700: F, t18: F, t2639: F, t342: F, t5202: F, t630: F, t231: F, t4129: F) -> (F, F, F, F, F) {
    let t18962 = t18961 * t3691;
    let t18968 = t2917 * t294;
    let t18969 = t18968 * t3700;
    let t18972 = t2639 * t18;
    let t18977 = t342 * t630 * t5202;
    let t18982 = t231 * t4129;
    (t18962, t18969, t18972, t18977, t18982)
}
