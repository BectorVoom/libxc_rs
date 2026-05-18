//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1321/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1321<F: Float>(t11427: F, t51306: F, t11423: F, t51351: F, t3116: F, t54373: F, t3065: F, t3840: F, t6645: F, t3879: F, t2134: F, t3759: F, t51214: F) -> (F, F, F, F, F, F) {
    let t56910 = t51306 * t11427;
    let t56912 = t51351 * t11423;
    let t56914 = t3116 * t54373;
    let t56916 = t3065 * t3840;
    let t56917 = t6645 * t56916;
    let t56919 = t3065 * t3879;
    let t56920 = t2134 * t56919;
    let t56922 = t51214 * t3759;
    (t56910, t56912, t56914, t56917, t56920, t56922)
}
