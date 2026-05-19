//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1125/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1125<F: Float>(t43686: F, t43690: F, t43693: F, t43695: F, t43698: F, t43699: F, t43708: F, t43712: F, t47340: F, t47341: F, t47344: F, t47347: F) -> F {
    let t47352 = t47340 - F::cast_from(0.46011511144704899612e1_f64) * t47341 - F::cast_from(0.19171462976960374838e0_f64) * t47344 + F::cast_from(0.14896037479937677779e-1_f64) * t47347 - t43686 + F::cast_from(0.71500979903700853338e0_f64) * t43690 + t43693 - t43695 - t43698 + F::cast_from(0.71500979903700853338e0_f64) * t43699 + t43708 + F::cast_from(0.19171462976960374838e0_f64) * t43712;
    t47352
}
