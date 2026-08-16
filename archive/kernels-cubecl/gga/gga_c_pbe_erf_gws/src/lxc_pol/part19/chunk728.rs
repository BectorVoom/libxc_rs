//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 728/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk728<F: Float>(t3989: F, t4146: F, t1162: F, t1178: F, t371: F, t1177: F, t1125: F, t4023: F, t3132: F, t3139: F, t4028: F, t1140: F, t1184: F) -> (F, F, F, F, F, F, F) {
    let t4147 = t3989 * t4146;
    let t4149 = t1178 * t1162;
    let t4150 = t371 * t4149;
    let t4151 = t1177 * t4150;
    let t4169 = t1125 * t4023;
    let t4171 = t3139 * t3132;
    let t4172 = t4028 * t4171;
    let t4174 = t1184 * t1140;
    (t4147, t4150, t4151, t4169, t4171, t4172, t4174)
}
