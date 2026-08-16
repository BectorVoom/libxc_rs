//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 459/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk459<F: Float>(t19: F, t2230: F, t601: F, t604: F, t84: F, t85: F, t24: F) -> (F, F, F, F) {
    let t2232 = F::cast_from(0.9492e2_f64) * t19 * t2230;
    let t2235 = t601 * t604;
    let t2239 = F::cast_from(1.0_f64) / t85 / t84;
    let t2240 = t24 * t2239;
    (t2232, t2235, t2239, t2240)
}
