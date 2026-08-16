//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 617/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk617<F: Float>(t3556: F, t448: F, t2756: F, t999: F, t3518: F, t535: F, t3560: F, t203: F, t3529: F) -> (F, F, F, F, F) {
    let t11154 = t3556 * t448;
    let t11157 = t999 * t2756;
    let t11160 = t535 * t3518;
    let t11163 = t3560 * t448;
    let t11167 = t203 * t3529;
    (t11154, t11157, t11160, t11163, t11167)
}
