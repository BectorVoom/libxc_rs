//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1213/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1213<F: Float>(t1195: F, t6729: F, t2222: F, t3955: F, t13953: F, t13976: F, t1176: F, t2298: F, t923: F, t13832: F, t51649: F, t867: F) -> (F, F, F, F, F) {
    let t51957 = F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t6729 * t1195;
    let t51958 = t3955 * t2222;
    let t51960 = t13953 * t13976;
    let t51963 = t1176 * t923 * t2298;
    let t51964 = t51963 * t13832;
    let t51966 = t51649 * t867;
    (t51957, t51958, t51960, t51964, t51966)
}
