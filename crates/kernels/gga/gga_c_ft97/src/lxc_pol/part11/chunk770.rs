//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 770/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk770<F: Float>(t10426: F, t2665: F, t446: F, t10286: F, t10391: F, t10394: F, t10398: F, t10400: F, t10404: F, t10407: F, t10412: F, t10417: F, t10420: F, t10424: F) -> (F, F, F) {
    let t10427 = t2665 * t10426;
    let t10428 = t446 * t10427;
    let t10430 = t10286 / F::cast_from(27.0_f64) - t10391 / F::cast_from(6.0_f64) + t10394 / F::cast_from(6.0_f64) - t10398 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10400 - t10404 / F::cast_from(3.0_f64) + t10407 / F::cast_from(3.0_f64) + t10412 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10417 - t10420 / F::cast_from(9.0_f64) + t10424 / F::cast_from(6.0_f64) + t10428 / F::cast_from(6.0_f64);
    (t10427, t10428, t10430)
}
