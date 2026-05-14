//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 725/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk725<F: Float>(t2621: F, t3096: F, t9438: F, t3340: F, t3424: F, t8998: F, t933: F, t2629: F, t1081: F, t2757: F, t2573: F, t3303: F, t1092: F, t2548: F, t2562: F, t327: F, t8820: F) -> (F, F, F, F, F, F, F) {
    let t9439 = t3096 * t2621;
    let t9440 = t9438 * t9439;
    let t9442 = t3424 * t3340;
    let t9444 = t933 * t8998;
    let t9445 = t9444 * t2629;
    let t9447 = t1081 * t2757;
    let t9449 = t3303 * t2573;
    let t9451 = t1092 * t2548;
    let t9454 = t8820 * t327 * t2562;
    (t9440, t9442, t9445, t9447, t9449, t9451, t9454)
}
