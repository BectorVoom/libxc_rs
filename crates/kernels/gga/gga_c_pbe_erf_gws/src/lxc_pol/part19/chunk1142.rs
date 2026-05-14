//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1142/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1142<F: Float>(t11701: F, t14015: F, t12088: F, t14007: F, t12050: F, t11656: F, t11782: F, t14069: F, t11502: F, t11829: F, t2407: F, t3116: F, t35207: F, t858: F, t14538: F, t3792: F, t51328: F) -> (F, F, F, F, F, F, F, F, F) {
    let t56861 = t14015 * t11701;
    let t56863 = t14007 * t12088;
    let t56865 = t14007 * t12050;
    let t56867 = t14007 * t11656;
    let t56869 = t11782 * t14069;
    let t56871 = t14007 * t11502;
    let t56873 = t14007 * t11829;
    let t56877 = t3116 * t2407 * t858 * t35207;
    let t56880 = t14538 * t51328 * t3792;
    (t56861, t56863, t56865, t56867, t56869, t56871, t56873, t56877, t56880)
}
