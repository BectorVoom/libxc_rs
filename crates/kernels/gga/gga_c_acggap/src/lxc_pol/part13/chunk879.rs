//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 879/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk879<F: Float>(t30374: F, t7499: F, t7315: F, t7828: F, t130: F, t1977: F, t7858: F, t7861: F, t2025: F, t7852: F, t593: F, t7510: F) -> (F, F, F, F, F, F) {
    let t30375 = t30374 * t7499;
    let t30387 = t7315 * t7828;
    let t30394 = t130 * t1977;
    let t30396 = t30394 * t7858 * t7861;
    let t30397 = F::new(5.0) / F::new(128.0) * t30396;
    let t30398 = t7852 * t2025;
    let t30400 = t593 * t7510;
    (t30375, t30387, t30394, t30397, t30398, t30400)
}
