//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 800/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk800<F: Float>(t3129: F, t31903: F, t9074: F, t10166: F, t9086: F, t6520: F, t6525: F, t7888: F, t2326: F, t3394: F, t6514: F, t30204: F, t31769: F) -> (F, F, F, F, F) {
    let t42587 = t9074 * t31903 * t3129;
    let t42590 = t9074 * t10166 * t9086;
    let t42640 = t6525 * t7888 * t6520;
    let t42644 = t9074 * t6514 * t3394 * t2326;
    let t42647 = t9074 * t30204 * t31769;
    (t42587, t42590, t42640, t42644, t42647)
}
