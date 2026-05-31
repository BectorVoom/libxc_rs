//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1018/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1018<F: Float>(t19752: F, t856: F, t91: F, t4191: F, t4226: F, t10631: F, t5337: F, t19246: F, t19249: F, t19252: F, t19255: F, t19258: F, t19261: F, t19265: F, t19269: F) -> (F, F, F, F) {
    let t19754 = t91 * t19752 * t856;
    let t19757 = t91 * t4191 * t4226;
    let t19759 = t10631 * t5337;
    let t19761 = t91 * t19759 * t856;
    let t19769 = t19246 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t19249 - t19754 / F::cast_from(12.0_f64) - t19757 / F::cast_from(6.0_f64) + t19761 / F::cast_from(8.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t19252 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t19255 - F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t19258 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t19261 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t19265 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t19269;
    (t19754, t19757, t19761, t19769)
}
