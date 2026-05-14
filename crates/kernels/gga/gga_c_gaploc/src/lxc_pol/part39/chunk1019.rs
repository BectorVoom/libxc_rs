//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1019/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1019<F: Float>(t12068: F, t1445: F, t1562: F, t2293: F, t11986: F, t2464: F, t2465: F, t587: F, t48086: F, t544: F, t9562: F, t2365: F, t38277: F, t4391: F, t42356: F, t42359: F, t42363: F, t42367: F, t42370: F, t42373: F, t42376: F, t42379: F) -> (F,) {
    let t48149 = t1562 * t1445 * t12068 * t2293;
    let t48154 = t587 * t2464 * t2465 * t11986;
    let t48156 = t544 * t48086;
    let t48157 = t48156 * t9562;
    let t48160 = t4391 * t2365 * t38277;
    let t48162 = -0.69017266717057349418e1 * t48149 + t42356 - t42359 + 0.43710935587469654631e2 * t42363 + 0.42603251059911944084e-1 * t48154 - 0.44688112439813033337e-1 * t48157 + 0.29792074959875355558e-1 * t48160 + t42367 + t42370 + t42373 - t42376 + t42379;
    (t48162,)
}
