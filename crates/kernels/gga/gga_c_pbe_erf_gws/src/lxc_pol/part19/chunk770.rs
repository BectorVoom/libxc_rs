//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 770/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk770<F: Float>(t197: F, t7698: F, t1802: F, t1885: F, t2566: F, t5129: F, t587: F, t2620: F, t597: F, t2646: F, t719: F, t256: F, t19: F, t2522: F, t336: F, t714: F) -> (F, F, F, F, F, F) {
    let t7699 = t7698 * t197;
    let t7703 = t1885 * t1802;
    let t7713 = t5129 * t2566;
    let t7715 = 16.0 / 135.0 * t587 * t7713;
    let t7720 = t2620 * t597;
    let t7726 = t2646 * t719;
    let t7728 = 2.0 / 3.0 * t7726 * t256;
    let t7729 = t2522 * t19;
    let t7730 = t7729 * t336;
    let t7732 = 0.12155555555555555555e0 * t7730 * t714;
    (t7699, t7703, t7715, t7720, t7728, t7732)
}
