//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 502/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk502<F: Float>(t2195: F, t858: F, t867: F, t866: F, t1477: F, t56: F) -> (F, F, F) {
    let t2196 = t858 * t2195;
    let t2197 = t867 * t2196;
    let t2199 = t866 * t2197 / F::cast_from(96.0_f64);
    let t2200 = t1477 * t56;
    (t2197, t2199, t2200)
}
