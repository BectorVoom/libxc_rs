//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 345/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk345<F: Float>(t2043: F, t571: F, t1494: F, t2011: F, t572: F, t1981: F, t552: F) -> (F, F, F, F, F) {
    let t2044 = t571 * t2043;
    let t2046 = t1494 * t2011;
    let t2047 = t572 * t2046;
    let t2048 = t571 * t2047;
    let t2050 = t1981 * t552;
    (t2044, t2046, t2047, t2048, t2050)
}
