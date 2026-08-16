//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 750/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk750<F: Float>(t1630: F, t219: F, t1811: F, t1620: F, t174: F, t177: F, t838: F, t1243: F, t574: F, t1760: F, t395: F, t1766: F) -> (F, F, F, F, F, F, F) {
    let t4934 = t1630 * t219;
    let t4935 = t4934 * t1811;
    let t4936 = t1620 * t4935;
    let t4939 = t174 * t838 * t177;
    let t4940 = F::cast_from(0.58774074074074074074e-2_f64) * t4939;
    let t4941 = t1243 * t574;
    let t4943 = t395 * t1760;
    let t4945 = t395 * t1766;
    (t4934, t4936, t4939, t4940, t4941, t4943, t4945)
}
