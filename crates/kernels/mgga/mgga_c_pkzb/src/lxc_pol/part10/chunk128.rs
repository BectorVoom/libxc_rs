//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 128/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk128<F: Float>(t336: F, t339: F, t342: F, t346: F) -> (F, F, F) {
    let t361 = 0.705945e1 * t339 + 0.1549425e1 * t336 + 0.420775e0 * t342 + 0.1562925e0 * t346;
    let t364 = 1.0 + 0.32163958997385070134e2 / t361;
    let t365 = f64::ln(t364);
    (t361, t364, t365)
}
