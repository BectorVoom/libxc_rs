//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1024/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1024<F: Float>(t3854: F, t6: F, t2171: F, t2345: F, t11459: F, t3139: F, t875: F, t2168: F, t2494: F, t343: F, t2170: F, t3131: F) -> (F, F, F, F, F, F) {
    let t11464 = t6 * t3854;
    let t11466 = t2345 * t11464 * t2171;
    let t11470 = t3139 * t11459 * t875;
    let t11472 = t2168 * t11470 / F::new(96.0);
    let t11473 = t343 * t2494;
    let t11475 = t2170 * t3131 * t11473;
    (t11464, t11466, t11470, t11472, t11473, t11475)
}
