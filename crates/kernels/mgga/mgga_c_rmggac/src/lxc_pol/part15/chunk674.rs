//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 674/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk674<F: Float>(t9737: F, t2060: F, t6557: F, t903: F, t1953: F, t71: F, t131: F, t638: F, t639: F, t2338: F, t574: F, t1950: F, t640: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9738 = F::new(0.85129199786595678796e-5) * t9737;
    let t9739 = t2060 * t6557;
    let t9740 = t903 * t9739;
    let t9741 = F::new(0.8980681276397856423e-1) * t9740;
    let t9745 = t71 * t1953;
    let t9746 = t9745 * t131;
    let t9748 = t638 * t639 * t9746;
    let t9749 = F::new(0.15243824895787514157e-3) * t9748;
    let t9750 = t2338 * t574;
    let t9752 = t638 * t639 * t9750;
    let t9753 = F::new(0.30487649791575028314e-3) * t9752;
    let t9754 = t640 * t1950;
    (t9738, t9739, t9741, t9745, t9746, t9749, t9750, t9753, t9754)
}
