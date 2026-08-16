//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1099/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1099<F: Float>(t245: F, t10174: F, t1580: F, t21: F, t2624: F, t267: F, t363: F, t37391: F, t41988: F, t43018: F, t5: F, t7745: F, t776: F) -> F {
    let t246 = F::cast_from(10000000.0_f64) <= t245;
    let t43034 = piecewise3::<F>(t246, F::cast_from(0.0_f64), t5 * (t41988 + t43018) * t21 / F::cast_from(4.0_f64) + t5 * t10174 * t363 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t5 * t2624 * t1580 + t5 * t776 * t7745 + t5 * t267 * t37391 / F::cast_from(4.0_f64));
    t43034
}
