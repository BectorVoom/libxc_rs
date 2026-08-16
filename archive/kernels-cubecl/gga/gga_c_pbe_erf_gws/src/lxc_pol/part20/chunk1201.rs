//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1201/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1201<F: Float>(t1105: F, t814: F, t3199: F, t898: F, t376: F, t745: F, t1114: F, t19905: F, t2409: F, t857: F, t338: F, t885: F) -> (F, F, F, F, F, F) {
    let t26623 = t1105 * t814;
    let t26654 = t3199 * t898;
    let t26730 = t376 * t745;
    let t26958 = t1114 * t19905;
    let t27047 = t857 * t2409;
    let t27105 = t885 * t338;
    (t26623, t26654, t26730, t26958, t27047, t27105)
}
