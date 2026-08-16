//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1028/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1028<F: Float>(t19782: F, t312: F, t19329: F, t19334: F, t19345: F, t19379: F, t19383: F, t19391: F, t19431: F, t19436: F, t19810: F, t19863: F) -> F {
    let t19886 = t19782 * t312;
    let t19898 = F::cast_from(2.0_f64) * t19886 - F::cast_from(2.0_f64) * t19334 - F::cast_from(4.0_f64) * t19345 + F::cast_from(8.0_f64) * t19383 - F::cast_from(4.0_f64) * t19329 + F::cast_from(4.0_f64) * t19810 - F::cast_from(12.0_f64) * t19431 + F::cast_from(8.0_f64) * t19436 - F::cast_from(2.0_f64) * t19391 + F::cast_from(4.0_f64) * t19379 - F::cast_from(2.0_f64) * t19863;
    t19898
}
