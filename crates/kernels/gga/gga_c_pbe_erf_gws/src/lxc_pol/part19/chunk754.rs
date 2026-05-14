//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 754/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk754<F: Float>(t343: F, t816: F, t874: F, t2251: F, t916: F, t2250: F, t339: F, t911: F, t824: F, t822: F, t56: F, t931: F, t19: F, t2132: F, t328: F, t2118: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6631 = t816 * t874 * t343;
    let t6636 = t2251 * t916;
    let t6637 = t2250 * t6636;
    let t6643 = t339 * t911;
    let t6644 = t824 * t6643;
    let t6645 = t822 * t6644;
    let t6658 = t56 * t931;
    let t6659 = t6658 * t19;
    let t6670 = t2132 * t328;
    let t6671 = t824 * t6670;
    let t6672 = t822 * t6671;
    let t6677 = t2118 * t6670;
    (t6631, t6636, t6637, t6643, t6644, t6645, t6659, t6671, t6672, t6677)
}
