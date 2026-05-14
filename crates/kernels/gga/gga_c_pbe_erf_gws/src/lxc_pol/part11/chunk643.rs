//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 643/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk643<F: Float>(t1072: F, t1472: F, t168: F, t2893: F, t501: F, t485: F, t974: F, t1508: F, t971: F, t1251: F, t1243: F, t2890: F, t1552: F, t978: F, t2863: F, t542: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8066 = t168 * t1472 * t1072;
    let t8122 = t501 * t2893;
    let t8135 = t485 * t974;
    let t8143 = t1508 * t971;
    let t8144 = t8143 * t1251;
    let t8149 = t2890 * t1243;
    let t8159 = t1552 * t978;
    let t8160 = t8159 * t1251;
    let t8197 = t2863 * t1243;
    let t8199 = t542 * t974;
    (t8066, t8122, t8135, t8143, t8144, t8149, t8159, t8160, t8197, t8199)
}
