//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1078/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1078<F: Float>(t50998: F, t52915: F, t9505: F, t14673: F, t2397: F, t3165: F, t376: F, t13796: F, t3989: F, t875: F, t1178: F, t904: F, t14637: F, t9292: F, t14688: F, t14802: F, t2408: F, t29751: F, t50904: F, t52889: F, t52893: F, t52897: F, t52902: F, t52904: F, t52908: F, t52910: F, t52912: F, t827: F) -> (F, F, F) {
    let t52917 = t50998 * t52915 * t9505;
    let t52919 = t14673 * t2397;
    let t52921 = t376 * t3165;
    let t52924 = t3989 * t13796 * t52921 * t875;
    let t52926 = t904 * t1178;
    let t52928 = t14637 * t52926 * t9292;
    let t52930 = t14688 * t2397;
    let t52931 = 7.0 / 144.0 * t52930;
    let t52935 = t52889 / 1536.0 + t52893 / 32.0 - t827 * t52897 / 48.0 - 7.0 / 144.0 * t50904 - t52902 + t52904 / 768.0 - t52908 / 192.0 - t52910 / 48.0 - t52912 / 48.0 + t52917 / 192.0 + t52919 / 48.0 - t52924 / 1536.0 - 5.0 / 384.0 * t52928 - t52931 - t2408 * t29751 * t14802 / 12.0;
    (t52921, t52926, t52935)
}
