//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 481/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk481<F: Float>(t2293: F, t2334: F, t1445: F, t9219: F, t203: F, t3085: F, t447: F, t1457: F, t9215: F, t9211: F, t3158: F, t528: F, t1645: F, t2349: F, t475: F, t188: F, t9181: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9309 = t2334 * t2293;
    let t9310 = t1445 * t9309;
    let t9313 = t1445 * t9219;
    let t9316 = t203 * t3085;
    let t9317 = t9316 * t447;
    let t9318 = t1445 * t9317;
    let t9321 = t1457 * t9219;
    let t9324 = t1457 * t9215;
    let t9327 = t1457 * t9211;
    let t9330 = t528 * t3158;
    let t9333 = t1645 * t2349;
    let t9338 = t9316 * t475;
    let t9339 = t1445 * t9338;
    let t9342 = t188 * t9181;
    (t9310, t9313, t9316, t9318, t9321, t9324, t9327, t9330, t9333, t9339, t9342)
}
