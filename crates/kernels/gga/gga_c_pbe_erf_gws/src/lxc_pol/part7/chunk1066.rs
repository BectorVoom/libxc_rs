//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1066/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1066<F: Float>(t1: F, t16192: F, t191: F, t745: F, t816: F, t21028: F, t858: F, t866: F, t867: F, t2168: F, t2170: F, t6177: F, t6220: F, t2121: F, t337: F, t5: F, t6084: F) -> (F, F, F, F, F) {
    let t21361 = t191 * t16192 * t1;
    let t21366 = t816 * t745;
    let t21378 = t866 * t867 * t858 * t21028 / 96.0;
    let t21382 = t2168 * t2170 * t6177 * t6220 / 8.0;
    let t21385 = t2121 * t337 * t5 * t6084;
    (t21361, t21366, t21378, t21382, t21385)
}
