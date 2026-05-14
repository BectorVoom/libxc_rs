//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 742/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk742<F: Float>(t2629: F, t9444: F, t1081: F, t2757: F, t2573: F, t3303: F, t1092: F, t2548: F, t2562: F, t327: F, t8820: F, t2560: F, t2568: F, t291: F, t7549: F, t7547: F) -> (F, F, F, F, F, F, F) {
    let t9445 = t9444 * t2629;
    let t9447 = t1081 * t2757;
    let t9449 = t3303 * t2573;
    let t9451 = t1092 * t2548;
    let t9454 = t8820 * t327 * t2562;
    let t9455 = t2560 * t9454;
    let t9457 = t2568 * t9454;
    let t9460 = t8820 * t291 * t7549;
    let t9461 = t7547 * t9460;
    (t9445, t9447, t9449, t9451, t9455, t9457, t9461)
}
