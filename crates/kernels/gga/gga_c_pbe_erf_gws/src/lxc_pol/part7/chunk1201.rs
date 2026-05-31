//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1201/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1201<F: Float>(t2168: F, t6269: F, t6523: F, t6524: F, t3138: F, t3139: F, t6177: F, t6360: F, t1: F, t16192: F, t191: F, t745: F, t816: F) -> (F, F, F, F) {
    let t21355 = F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t2168 * t6523 * t6269 * t6524;
    let t21359 = F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t3138 * t3139 * t6177 * t6360;
    let t21361 = t191 * t16192 * t1;
    let t21366 = t816 * t745;
    (t21355, t21359, t21361, t21366)
}
