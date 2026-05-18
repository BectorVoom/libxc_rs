//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 744/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk744<F: Float>(t470: F, t4788: F, t1392: F, t1399: F, t1422: F, t75: F, t472: F, t1218: F, t1327: F, t414: F, t1319: F, t455: F) -> (F, F, F, F, F, F) {
    let t4789 = t470 * t4788;
    let t4790 = F::new(0.58482233974552040708e0) * t4789;
    let t4791 = t1399 * t1392;
    let t4793 = t1422 * t75;
    let t4794 = t4793 * t472;
    let t4796 = t1399 * t1218;
    let t4798 = t414 * t1327;
    let t4799 = F::new(12.0) * t4798;
    let t4800 = t1319 * t455;
    (t4790, t4791, t4794, t4796, t4799, t4800)
}
