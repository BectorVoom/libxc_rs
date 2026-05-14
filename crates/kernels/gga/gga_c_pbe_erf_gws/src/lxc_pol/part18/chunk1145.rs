//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1145/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1145<F: Float>(t14696: F, t39061: F, t3972: F, t3975: F, t4182: F, t8589: F, t829: F, t830: F, t38036: F, t6472: F, t820: F, t15272: F, t2376: F, t14767: F, t3047: F, t28652: F, t3808: F) -> (F, F, F, F, F, F) {
    let t56657 = t3972 * t3975 * t39061 * t14696;
    let t56659 = t8589 * t4182;
    let t56661 = t829 * t830 * t56659;
    let t56667 = t3972 * t3975 * t38036 * t6472 * t820;
    let t56669 = t2376 * t15272;
    let t56671 = t829 * t830 * t56669;
    let t56674 = t14767 * t3047;
    let t56678 = t3972 * t3975 * t3808 * t28652;
    (t56657, t56661, t56667, t56671, t56674, t56678)
}
