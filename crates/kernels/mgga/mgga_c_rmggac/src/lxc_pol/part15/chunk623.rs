//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 623/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk623<F: Float>(t9732: F, t589: F, t597: F, t201: F, t1979: F, t1982: F, t2060: F, t6557: F, t903: F, t1953: F, t71: F, t131: F, t638: F, t639: F, t2338: F, t574: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9733 = 0.42564599893297839398e-5 * t9732;
    let t9734 = t589 * t597;
    let t9735 = t9734 * t201;
    let t9737 = t9735 * t1979 * t1982;
    let t9738 = 0.85129199786595678796e-5 * t9737;
    let t9739 = t2060 * t6557;
    let t9740 = t903 * t9739;
    let t9741 = 0.8980681276397856423e-1 * t9740;
    let t9745 = t71 * t1953;
    let t9746 = t9745 * t131;
    let t9748 = t638 * t639 * t9746;
    let t9749 = 0.15243824895787514157e-3 * t9748;
    let t9750 = t2338 * t574;
    (t9733, t9734, t9735, t9738, t9739, t9741, t9745, t9746, t9749, t9750)
}
