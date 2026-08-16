//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1025/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1025<F: Float>(t19298: F, t19301: F, t19304: F, t10797: F, t19273: F, t19276: F, t19283: F, t19287: F, t19292: F, t19295: F, t19852: F, t19826: F, t19836: F, t19849: F) -> F {
    let t19857 = t19298 / F::cast_from(9.0_f64);
    let t19858 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t19301;
    let t19859 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t19304;
    let t19860 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t19273 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t19276 - t19852 - t10797 + F::cast_from(2.0_f64) * t19283 - t19287 / F::cast_from(3.0_f64) - F::cast_from(6.0_f64) * t19292 + F::cast_from(4.0_f64) * t19295 + t19857 - t19858 + t19859;
    let t19862 = t19826 + t19836 + t19849 + t19860;
    t19862
}
