//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1022/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1022<F: Float>(t8859: F, t8918: F, t8955: F, t9003: F, t9034: F, t9106: F, t9139: F, t9197: F, t339: F, t338: F, t376: F, t1144: F, t2353: F) -> (F, F, F) {
    let t9200 = t8859 + t8918 + t8955 + t9003 + t9034 + t9106 + t9139 + t9197;
    let t9201 = t339 * t9200;
    let t9203 = t338 * t9201 * t376;
    let t9208 = t338 * t1144 * t2353;
    (t9201, t9203, t9208)
}
