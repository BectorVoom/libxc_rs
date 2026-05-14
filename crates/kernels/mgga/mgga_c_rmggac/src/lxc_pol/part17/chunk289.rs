//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 289/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk289<F: Float>(t1368: F, t570: F, t552: F, t558: F, t1598: F, t521: F, t1714: F) -> (F, F, F, F, F) {
    let t1767 = t1368 * t570;
    let t1773 = t552 * t558;
    let t1776 = t1598 * t570;
    let t1794 = t521 * t521;
    let t1797 = 2.0 * t1714;
    (t1767, t1773, t1776, t1794, t1797)
}
