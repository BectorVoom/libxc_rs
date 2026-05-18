//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 807/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk807<F: Float>(t12664: F, t15362: F, t28594: F, t7785: F, t12705: F, t7416: F, t10012: F, t2530: F, t2684: F, t9438: F, t12657: F, t23157: F) -> (F, F, F, F, F) {
    let t41305 = t15362 * t12664;
    let t41307 = t28594 * t7785;
    let t41312 = t7416 * t12705;
    let t41316 = t2684 * t9438 * t10012 * t2530;
    let t41330 = t23157 * t12657;
    (t41305, t41307, t41312, t41316, t41330)
}
