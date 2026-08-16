//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3467/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3467<F: Float>(t1063: F, t11986: F, t247: F, t6100: F, t20054: F, t3106: F, t3075: F, t5819: F, t2251: F, t5825: F) -> (F, F, F, F) {
    let t65357 = t1063 * t247 * t11986 * t6100;
    let t65359 = t3106 * t20054;
    let t65365 = t5819 * t3075;
    let t65370 = t5825 * t2251;
    (t65357, t65359, t65365, t65370)
}
