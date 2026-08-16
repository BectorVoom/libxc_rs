//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 892/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk892(t16712: f64, t197: f64, t16669: f64, t5293: f64, t587: f64, t1820: f64, t5018: f64, t5300: f64, t16907: f64, t16910: f64, t16912: f64, t16917: f64, t16921: f64, t16925: f64, t16927: f64, t16929: f64, t16931: f64) -> (f64, f64, f64) {
    let t16932 = t197 * t16712;
    let t16936 = 128.0_f64 / 27.0_f64 * t587 * t5293 * t16932 * t16669;
    let t16938 = t1820 * t5018 * t5300;
    let t16939 = 64.0_f64 / 15.0_f64 * t16938;
    let t16940 = -t16907 - t16910 - t16912 - t16917 + t16921 + t16925 + t16927 - t16929 - t16931 - t16936 + t16939;
    (t16936, t16939, t16940)
}
