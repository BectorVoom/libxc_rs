//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 453/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk453<F: Float>(t5: F, t874: F, t343: F, t337: F, t2121: F, t838: F, t855: F, t859: F) -> (F, F, F, F) {
    let t2135 = t5 * t874;
    let t2136 = t2135 * t343;
    let t2137 = t337 * t2136;
    let t2138 = t2121 * t2137;
    let t2141 = t855 * t838;
    let t2142 = t2141 * t859;
    (t2136, t2137, t2138, t2142)
}
