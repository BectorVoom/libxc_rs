//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 662/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk662<F: Float>(t16879: F, t486: F, t165: F, t2089: F, t16534: F, t169: F, t10913: F, t2021: F, t1423: F, t7784: F, t1964: F, t9419: F, t823: F, t40: F, t7291: F, t10007: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21501 = t16879 * t486;
    let t21502 = t165 * t2089;
    let t22090 = t16534 * t169;
    let t22242 = t2021 * t10913;
    let t22256 = t1423 * t7784;
    let t22537 = t1964 * t9419;
    let t22542 = t823 * t9419;
    let t22623 = t40 * t2089;
    let t22624 = t22623 * t7291;
    let t22629 = t10007 * t7291;
    (t21501, t21502, t22090, t22242, t22256, t22537, t22542, t22623, t22624, t22629)
}
