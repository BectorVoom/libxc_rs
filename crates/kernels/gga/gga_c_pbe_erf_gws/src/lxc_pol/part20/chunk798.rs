//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 798/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk798<F: Float>(t362: F, t922: F, t2276: F, t932: F, t2132: F, t2306: F, t1477: F, t863: F, t864: F, t877: F, t2263: F, t328: F) -> (F, F, F, F, F, F) {
    let t6201 = t362 * t922;
    let t6203 = t2276 * t6201 * t932;
    let t6216 = t2306 * t2132;
    let t6228 = t863 * t864 * t1477;
    let t6229 = t6228 * t877;
    let t6238 = t2263 * t328;
    (t6201, t6203, t6216, t6228, t6229, t6238)
}
