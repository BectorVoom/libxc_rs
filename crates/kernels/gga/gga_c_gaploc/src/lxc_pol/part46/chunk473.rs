//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 473/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk473<F: Float>(t123: F, t6514: F, t2326: F, t9074: F, t4261: F, t6510: F, t447: F, t9198: F, t1064: F, t2293: F, t2344: F, t2343: F, t9193: F, t1437: F, t3158: F, t2304: F, t2349: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9204 = t6514 * t123;
    let t9205 = t9204 * t2326;
    let t9207 = 0.71137516589190373998e-2 * t9074 * t9205;
    let t9208 = t4261 * t6510;
    let t9210 = 0.47425011059460249332e-2 * t9074 * t9208;
    let t9211 = t9198 * t447;
    let t9212 = t1064 * t9211;
    let t9215 = t2344 * t2293;
    let t9216 = t2343 * t9215;
    let t9219 = t9193 * t447;
    let t9220 = t2343 * t9219;
    let t9223 = t3158 * t1437;
    let t9226 = t2304 * t2349;
    (t9204, t9207, t9210, t9211, t9212, t9215, t9216, t9219, t9220, t9223, t9226)
}
