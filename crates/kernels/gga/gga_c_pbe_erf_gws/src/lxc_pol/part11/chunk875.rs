//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 875/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk875<F: Float>(t1243: F, t3466: F, t3474: F, t3470: F, t17791: F, t3406: F, t639: F, t3443: F, t5219: F, t10968: F, t586: F, t1672: F, t211: F, t3554: F, t3499: F, t5463: F) -> (F, F, F, F, F, F, F, F) {
    let t30955 = t1243 * t3466;
    let t30957 = t1243 * t3474;
    let t30962 = t1243 * t3470;
    let t31102 = t639 * t17791 * t3406;
    let t31133 = t5219 * t3443;
    let t31168 = t10968 * t586;
    let t31200 = t211 * t1672 * t3554;
    let t31225 = t639 * t5463 * t3499;
    (t30955, t30957, t30962, t31102, t31133, t31168, t31200, t31225)
}
