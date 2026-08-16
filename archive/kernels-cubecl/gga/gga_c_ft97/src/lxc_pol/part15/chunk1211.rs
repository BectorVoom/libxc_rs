//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1211/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1211<F: Float>(t44776: F, t71239: F, t71277: F, t71299: F, t71306: F, t71320: F, t83722: F, t83728: F, t83770: F, t83772: F, t83781: F, t83789: F, t83792: F, t90335: F) -> F {
    let t91195 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t83722 + t71239 + F::cast_from(20.0_f64) / F::cast_from(243.0_f64) * t83728 + t71277 + t44776 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t90335 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t83770 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t83772 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t83781 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t83789 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t83792 - t71299 + t71306 - t71320;
    t91195
}
