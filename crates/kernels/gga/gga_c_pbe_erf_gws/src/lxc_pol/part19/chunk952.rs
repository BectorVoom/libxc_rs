//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 952/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk952<F: Float>(t2345: F, t3814: F, t9375: F, t11732: F, t858: F, t867: F, t866: F, t11737: F, t2210: F, t884: F, t2164: F, t3880: F, t11363: F, t6384: F, t904: F, t11889: F, t2300: F) -> (F, F, F, F, F, F) {
    let t11901 = t2345 * t9375 * t3814;
    let t11905 = t867 * t858 * t11732;
    let t11907 = t866 * t11905 / 96.0;
    let t11909 = t2210 * t858 * t11737;
    let t11911 = t884 * t11909 / 16.0;
    let t11912 = t2164 * t3880;
    let t11913 = 7.0 / 288.0 * t11912;
    let t11915 = t6384 * t904 * t11363;
    let t11919 = t2300 * t904 * t11889;
    (t11901, t11907, t11911, t11913, t11915, t11919)
}
