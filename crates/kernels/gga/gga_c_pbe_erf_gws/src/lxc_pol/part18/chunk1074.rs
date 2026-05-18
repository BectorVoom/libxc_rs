//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1074/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1074<F: Float>(t12065: F, t8848: F, t1139: F, t2169: F, t856: F, t3108: F, t1076: F, t1112: F, t2118: F, t3074: F, t1185: F, t346: F, t825: F) -> (F, F, F, F, F) {
    let t12067 = t8848 * t12065 / F::new(96.0);
    let t12068 = t2169 * t1139;
    let t12069 = t856 * t12068;
    let t12071 = t3108 * t12069 / F::new(24.0);
    let t12072 = t1112 * t1076;
    let t12073 = t2118 * t12072;
    let t12074 = t3074 * t12073;
    let t12076 = t346 * t825 * t1185;
    (t12067, t12071, t12072, t12074, t12076)
}
