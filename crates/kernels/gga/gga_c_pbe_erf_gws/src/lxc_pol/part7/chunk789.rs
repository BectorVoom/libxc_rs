//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 789/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk789<F: Float>(t2285: F, t2289: F, t6232: F, t858: F, t867: F, t866: F, t3205: F, t336: F, t2182: F, t343: F, t2135: F, t2168: F) -> (F, F, F, F, F, F, F) {
    let t6517 = t2289 * t2285;
    let t6520 = t867 * t858 * t6232;
    let t6522 = t866 * t6520 / F::cast_from(96.0_f64);
    let t6523 = t3205 * t336;
    let t6524 = t343 * t2182;
    let t6526 = t6523 * t2135 * t6524;
    let t6528 = F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t2168 * t6526;
    (t6517, t6520, t6522, t6523, t6524, t6526, t6528)
}
