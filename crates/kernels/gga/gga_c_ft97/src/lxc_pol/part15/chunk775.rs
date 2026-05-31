//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 775/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk775<F: Float>(t21369: F, t683: F, t92: F, t13538: F, t18096: F, t18107: F, t18115: F, t21353: F, t21357: F, t21360: F, t21364: F, t21367: F, t9557: F) -> (F, F, F) {
    let t21370 = t683 * t21369;
    let t21371 = t92 * t21370;
    let t21373 = -t9557 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t13538 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t18096 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t18107 + t18115 / F::cast_from(3.0_f64) - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t21353 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t21357 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t21360 - F::cast_from(2.0_f64) * t21364 + F::cast_from(2.0_f64) * t21367 - t21371 / F::cast_from(3.0_f64);
    (t21370, t21371, t21373)
}
