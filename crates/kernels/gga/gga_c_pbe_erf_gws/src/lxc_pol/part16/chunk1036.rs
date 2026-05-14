//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1036/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1036<F: Float>(t14873: F, t14899: F, t14940: F, t14967: F, t14985: F, t15000: F, t15016: F, t15094: F, t2053: F, t4233: F, t944: F, t1167: F, t14364: F, t3324: F, t4120: F, t360: F, t898: F) -> (F, F, F, F, F, F) {
    let t15097 = t14873 + t14899 + t14940 + t14967 + t14985 + t15000 + t15016 + t15094;
    let t15101 = t4233 * t2053;
    let t15102 = t15101 * t944;
    let t15108 = t14364 * t1167;
    let t15113 = t4120 * t3324;
    let t15636 = t898 * t360;
    (t15097, t15101, t15102, t15108, t15113, t15636)
}
