//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 503/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk503<F: Float>(t5053: F, t676: F, t27: F, t89: F, t2335: F, t3942: F, t3947: F, t4920: F, t4924: F, t4928: F, t4932: F, t4937: F) -> (F, F, F) {
    let t5054 = t676 * t5053;
    let t5056 = t89 * t27 * t5054;
    let t5058 = t2335 + t3942 + t3947 - t4920 / F::cast_from(27.0_f64) + t4924 / F::cast_from(9.0_f64) + t4928 / F::cast_from(9.0_f64) - t4932 / F::cast_from(18.0_f64) + t4937 / F::cast_from(3.0_f64) - t5056 / F::cast_from(6.0_f64);
    (t5054, t5056, t5058)
}
