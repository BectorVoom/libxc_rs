//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1249/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1249<F: Float>(t13808: F, t14596: F, t52036: F, t15097: F, t945: F, t1172: F, t1211: F, t319: F, t4233: F, t6854: F, t321: F, t318: F) -> (F, F, F, F, F, F, F) {
    let t54730 = t13808 * t14596;
    let t54737 = F::new(35.0) / F::new(216.0) * t52036;
    let t54766 = t15097 * t945;
    let t54778 = t1172 * t319 * t1211;
    let t54792 = t4233 * t6854;
    let t54797 = F::new(2.0) * t321 * t54766;
    let t54802 = t1172 * t318 * t1211;
    (t54730, t54737, t54766, t54778, t54792, t54797, t54802)
}
