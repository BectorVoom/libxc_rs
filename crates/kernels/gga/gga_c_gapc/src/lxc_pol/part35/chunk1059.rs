//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1059/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1059<F: Float>(t11798: F, t16720: F, t3284: F, t11387: F, t16676: F, t16677: F, t11794: F, t7420: F, t11320: F, t2619: F, t7921: F, t11499: F, t2629: F, t933: F) -> (F, F, F, F, F) {
    let t33170 = t11798 * t3284 * t16720;
    let t33173 = t16676 * t11387 * t16677;
    let t33175 = t11794 * t7420;
    let t33179 = t2619 * t11320 * t7921;
    let t33182 = t933 * t11499 * t2629;
    (t33170, t33173, t33175, t33179, t33182)
}
