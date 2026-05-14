//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 946/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk946<F: Float>(t2133: F, t3916: F, t2138: F, t3111: F, t3763: F, t2255: F, t1109: F, t745: F, t3258: F, t3717: F, t5: F, t337: F, t2147: F, t2146: F, t2164: F, t3832: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11794 = t3916 * t2133;
    let t11796 = t11794 * t2138 / 96.0;
    let t11797 = t3111 * t3763;
    let t11798 = t2255 * t11797;
    let t11801 = t745 * t1109;
    let t11803 = t2255 * t3258 * t11801;
    let t11806 = t5 * t3717;
    let t11807 = t337 * t11806;
    let t11808 = t2147 * t11807;
    let t11810 = t2146 * t11808 / 48.0;
    let t11811 = t2164 * t3832;
    (t11794, t11796, t11797, t11798, t11803, t11806, t11807, t11810, t11811)
}
