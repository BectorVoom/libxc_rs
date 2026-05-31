//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 630/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk630<F: Float>(t295: F, t312: F, t5374: F, t1248: F, t4246: F, t296: F, t2793: F, t4032: F, t4049: F, t5211: F, t5215: F, t5219: F, t5223: F, t5228: F, t5302: F, t5339: F, t5364: F) -> (F, F, F, F) {
    let t5376 = t295 * t5374 * t312;
    let t5380 = t4246 * t1248;
    let t5381 = t296 * t5380;
    let t5393 = -t5339 / F::cast_from(4.0_f64) + t5364 / F::cast_from(2.0_f64) + t2793 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4032 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t4049 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5211 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5215 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5219 - t5223 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) * t5228 - t5302;
    (t5376, t5380, t5381, t5393)
}
