//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 544/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk544<F: Float>(t299: F, t977: F, t278: F, t253: F, t330: F) -> (F, F, F) {
    let t2835 = F::cast_from(1.0_f64) / t977 / t299;
    let t2836 = t278 * t2835;
    let t2839 = t253 * t330;
    let t2840 = F::cast_from(1.0_f64) / t2839;
    (t2835, t2836, t2840)
}
