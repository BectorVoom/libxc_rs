//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 781/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk781<F: Float>(t2365: F, t31586: F, t4391: F, t31591: F, t12960: F, t31051: F, t10473: F, t2478: F, t6576: F, t34688: F, t9272: F, t9273: F) -> (F, F, F, F, F) {
    let t41626 = t4391 * t2365 * t31586;
    let t41629 = t4391 * t2365 * t31591;
    let t41645 = t31051 * t12960;
    let t41649 = t6576 * t10473 * t2478;
    let t41656 = t9272 * t34688 * t9273;
    (t41626, t41629, t41645, t41649, t41656)
}
