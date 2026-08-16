//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 834/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk834<F: Float>(t1255: F, t4969: F, t835: F, t14715: F, t21947: F, t21951: F, t21955: F, t21960: F, t21964: F, t21967: F, t21971: F, t21975: F, t21984: F, t21987: F, t21991: F, t21994: F) -> (F, F) {
    let t22261 = t835 * t1255 * t4969;
    let t22275 = F::cast_from(2.0_f64) * t21994 + t21971 + t21975 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t21960 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t21967 - F::cast_from(2.0_f64) * t21947 - F::cast_from(2.0_f64) * t21951 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t21955 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t21964 + F::cast_from(6.0_f64) * t21984 - t21987 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) * t21991 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t14715;
    (t22261, t22275)
}
