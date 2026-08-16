//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3124/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3124<F: Float>(t1248: F, t16750: F, t12915: F, t17344: F, t17345: F, t247: F, t1260: F, t44843: F, t17423: F, t17426: F, t11249: F, t5284: F) -> (F, F, F, F, F) {
    let t57498 = t16750 * t1248;
    let t57508 = t17344 * t247 * t12915 * t17345;
    let t57520 = t44843 * t1260;
    let t57534 = t17426 * t17423;
    let t57536 = t5284 * t11249;
    (t57498, t57508, t57520, t57534, t57536)
}
