//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1725/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1725<F: Float>(t1695: F, t3268: F, t12230: F, t1732: F, t3495: F, t5180: F, t3302: F, t5332: F, t1716: F, t2435: F) -> (F, F, F, F, F) {
    let t16604 = t3268 * t1695;
    let t16668 = t1732 * t12230;
    let t16676 = t3495 * t5180;
    let t16695 = t5332 * t3302;
    let t16706 = t2435 * t1716;
    (t16604, t16668, t16676, t16695, t16706)
}
