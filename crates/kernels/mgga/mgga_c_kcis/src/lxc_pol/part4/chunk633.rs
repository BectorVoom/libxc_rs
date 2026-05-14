//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 633/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk633<F: Float>(t1319: F, t518: F, t1419: F, t3786: F, t237: F, t334: F, t451: F) -> (F, F, F) {
    let t3787 = t518 * t1319;
    let t3788 = t3787 * t1419;
    let t3789 = t3786 * t3788;
    let t3793 = t237 * t334 * t451;
    (t3788, t3789, t3793)
}
