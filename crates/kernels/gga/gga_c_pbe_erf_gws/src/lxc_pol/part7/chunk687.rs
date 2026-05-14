//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 687/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk687<F: Float>(t156: F, t1999: F, t670: F, t1: F, t1354: F, t3: F, t672: F, t1996: F, t2000: F, t2007: F, t671: F, t703: F, t2003: F, t666: F, t678: F, t762: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5931 = t156 * t1999;
    let t5933 = 0.21642082724729686754e0 * t670 * t5931;
    let t5935 = t1354 * t1 * t3;
    let t5936 = t5935 * t672;
    let t5938 = t1996 * t2000;
    let t5940 = t1996 * t2007;
    let t5942 = t703 * t671;
    let t5944 = 0.11181742741110338156e-1 * t2003 * t5942;
    let t5945 = t666 * t678;
    let t5948 = 0.11033703703703703703e-2 * t762 * t671;
    (t5931, t5933, t5935, t5936, t5938, t5940, t5942, t5944, t5945, t5948)
}
