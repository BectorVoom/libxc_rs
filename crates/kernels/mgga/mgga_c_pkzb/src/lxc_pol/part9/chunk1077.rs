//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1077/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1077<F: Float>(t5236: F, t5257: F, t1732: F, t6895: F, t167: F, t168: F, t16942: F, t180: F, t66: F, t5221: F, t5261: F, t16405: F) -> (F, F, F, F, F) {
    let t17056 = t5257 * t5236;
    let t17067 = t6895 * t1732;
    let t17088 = F::new(0.28974367305964659283e0) * t167 * t168 / t66 / t16942 * t180;
    let t17089 = t5221 * t5261;
    let t17095 = t167 * t16405;
    (t17056, t17067, t17088, t17089, t17095)
}
