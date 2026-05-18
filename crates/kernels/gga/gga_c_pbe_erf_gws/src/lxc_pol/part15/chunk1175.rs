//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1175/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1175<F: Float>(t1114: F, t19905: F, t2409: F, t857: F, t338: F, t885: F, t2219: F, t2416: F, t2182: F, t3178: F, t5: F, t9079: F) -> (F, F, F, F, F, F) {
    let t26958 = t1114 * t19905;
    let t27047 = t857 * t2409;
    let t27105 = t885 * t338;
    let t27112 = t2219 * t2416;
    let t27363 = t3178 * t2182;
    let t27618 = t5 * t9079;
    (t26958, t27047, t27105, t27112, t27363, t27618)
}
