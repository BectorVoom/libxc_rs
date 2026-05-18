//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 801/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk801<F: Float>(t18265: F, t18266: F, t18381: F, t18382: F, t18383: F, t21406: F, t21410: F, t21440: F, t21444: F, t21451: F, t21455: F, t21459: F, t9972: F) -> F {
    let t21716 = -t18265 + t18266 - F::new(2.0) / F::new(3.0) * t21406 - F::new(2.0) / F::new(3.0) * t21410 + t21459 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t21440 + F::new(2.0) / F::new(9.0) * t21444 - F::new(2.0) / F::new(9.0) * t21451 + t21455 / F::new(3.0) + t18381 - t18382 + t18383 - t9972;
    t21716
}
