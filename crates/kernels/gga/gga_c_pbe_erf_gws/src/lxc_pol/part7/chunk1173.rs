//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1173/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1173<F: Float>(t20930: F, t366: F, t899: F, t2157: F, t20726: F, t2264: F, t2331: F, t2268: F, t2276: F, t2299: F, t6201: F, t6581: F) -> (F, F, F, F) {
    let t20932 = t899 * t20930 * t366;
    let t20933 = t2157 * t2157;
    let t20934 = t20726 * t20933;
    let t20940 = t899 * t2264 * t2331;
    let t20941 = t20940 * t2268;
    let t20944 = t2276 * t6201 * t2299;
    let t20945 = t20944 * t6581;
    (t20932, t20934, t20941, t20945)
}
