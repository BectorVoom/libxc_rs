//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 674/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk674<F: Float>(t1247: F, t3103: F, t12380: F, t464: F, t866: F, t1233: F, t157: F, t883: F, t9193: F, t6485: F, t882: F, t3158: F, t6470: F) -> (F, F, F, F, F, F, F, F) {
    let t12397 = t1247 * t3103;
    let t12399 = t464 * t12380;
    let t12400 = t12399 * t866;
    let t12411 = F::cast_from(1.0_f64) / t1233;
    let t12412 = t157 * t12411;
    let t12423 = t883 * t9193;
    let t12424 = t6485 * t12423;
    let t12425 = t882 * t12424;
    let t12427 = t3158 * t6470;
    (t12397, t12399, t12400, t12411, t12412, t12424, t12425, t12427)
}
