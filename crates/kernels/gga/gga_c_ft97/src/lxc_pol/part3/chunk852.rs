//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 852/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk852<F: Float>(t16745: F, t16748: F, t16751: F, t2: F, t4714: F, t1985: F, t558: F, t4668: F, t9016: F, t3408: F, t3518: F, t16395: F, t582: F) -> (F, F, F, F, F, F, F) {
    let t17249 = t16745 / F::cast_from(9.0_f64);
    let t17250 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t16748;
    let t17251 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t16751;
    let t17254 = t2 * t4714;
    let t17256 = t1985 * t17254 * t558;
    let t17259 = t2 * t4668;
    let t17261 = t9016 * t17259 * t558;
    let t17265 = t1985 * t3518 * t3408;
    let t17268 = t582 * t16395;
    (t17249, t17250, t17251, t17256, t17261, t17265, t17268)
}
