//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 792/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk792<F: Float>(t6520: F, t6525: F, t7888: F, t2326: F, t3394: F, t6514: F, t9074: F, t30204: F, t31769: F, t10177: F, t19531: F, t883: F) -> (F, F, F, F) {
    let t42640 = t6525 * t7888 * t6520;
    let t42644 = t9074 * t6514 * t3394 * t2326;
    let t42647 = t9074 * t30204 * t31769;
    let t42651 = t9074 * t19531 * t883 * t10177;
    (t42640, t42644, t42647, t42651)
}
