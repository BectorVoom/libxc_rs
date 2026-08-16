//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1268/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1268<F: Float>(t15386: F, t321: F, t15382: F, t945: F, t14767: F, t2503: F, t4002: F, t9955: F, t29260: F, t3808: F, t3972: F, t3975: F) -> (F, F, F, F, F, F) {
    let t56057 = t321 * t15386;
    let t56059 = t15382 * t945;
    let t56060 = t321 * t56059;
    let t56061 = t14767 * t2503;
    let t56063 = t9955 * t4002;
    let t56067 = t3972 * t3975 * t3808 * t29260;
    (t56057, t56059, t56060, t56061, t56063, t56067)
}
