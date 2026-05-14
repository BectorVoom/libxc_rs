//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 715/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk715<F: Float>(t188: F, t9: F, t1887: F, t1820: F, t1718: F, t401: F, t1699: F, t395: F, t191: F, t784: F, t190: F, t212: F, t1251: F, t658: F, t1721: F, t1715: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5018 = t9 * t188;
    let t5019 = t5018 * t1887;
    let t5020 = t1820 * t5019;
    let t5022 = t401 * t1718;
    let t5042 = t395 * t1699;
    let t5044 = t784 * t191;
    let t5047 = 0.29629629629629629629e-1 * t190 * t5044 * t212;
    let t5052 = t1251 * t658;
    let t5054 = t401 * t1721;
    let t5056 = t401 * t1715;
    (t5018, t5020, t5022, t5042, t5044, t5047, t5052, t5054, t5056)
}
