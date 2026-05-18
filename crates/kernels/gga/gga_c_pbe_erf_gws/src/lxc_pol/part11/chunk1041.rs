//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1041/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1041<F: Float>(t11348: F, t3047: F, t1144: F, t12180: F, t4386: F, t13126: F, t19563: F, t13662: F, t28397: F, t3083: F, t9885: F, t3052: F) -> (F, F, F, F, F, F) {
    let t44093 = t11348 * t3047;
    let t44104 = t4386 * t1144 * t12180;
    let t44115 = t13126 * t19563;
    let t44118 = t28397 * t13662;
    let t44131 = t3083 * t9885;
    let t44138 = t11348 * t3052;
    (t44093, t44104, t44115, t44118, t44131, t44138)
}
