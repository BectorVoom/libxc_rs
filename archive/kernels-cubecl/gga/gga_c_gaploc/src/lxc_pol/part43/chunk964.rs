//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 964/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk964<F: Float>(t2617: F, t3726: F, t7803: F, t2679: F, t9796: F, t12240: F, t7810: F, t38961: F, t935: F, t1457: F, t2103: F, t12213: F, t2530: F) -> (F, F, F, F, F, F) {
    let t47206 = t7803 * t3726 * t2617;
    let t47212 = t9796 * t3726 * t2679;
    let t47215 = t7810 * t12240 * t2617;
    let t47220 = t38961 * t935;
    let t47222 = t2103 * t1457 * t47220;
    let t47225 = t12213 * t2530;
    (t47206, t47212, t47215, t47220, t47222, t47225)
}
