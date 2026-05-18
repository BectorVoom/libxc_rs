//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1002/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1002<F: Float>(t2508: F, t28668: F, t43191: F, t5241: F, t13176: F, t2549: F, t43107: F, t7290: F, t1841: F, t7289: F, t40822: F, t40825: F) -> (F, F, F, F, F, F) {
    let t43195 = F::new(0.46143157380853345701e0) * t2508 * t43191 * t5241 * t28668;
    let t43196 = t2549 * t13176;
    let t43199 = t7290 * t43107;
    let t43202 = F::new(0.17090058289204942852e-2) * t1841 * t7289 * t43199;
    let t43203 = F::new(0.1922631557535556071e-2) * t40822;
    let t43204 = F::new(0.3845263115071112142e-2) * t40825;
    (t43195, t43196, t43199, t43202, t43203, t43204)
}
