//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1234/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1234<F: Float>(t24: F, t1429: F, t507: F, t8: F, t1165: F, t1541: F, t1652: F, t1655: F, t19863: F, t3019: F, t3725: F, t3727: F, t4803: F, t78: F, t821: F, t9784: F, t9789: F, t9792: F, zeta_threshold: F) -> (F, F) {
    let t90 = t24 <= zeta_threshold;
    let t23971 = t507 * t8 * t1429;
    let t23990 = piecewise3(t90, 0.0, -56.0 / 81.0 * t9784 * t1652 - 64.0 / 27.0 * t3019 * t23971 + 8.0 / 27.0 * t3725 * t1655 - 16.0 / 9.0 * t821 * t78 * t1541 + 8.0 / 9.0 * t1165 * t1429 - 8.0 / 3.0 * t1165 * t4803 + 8.0 / 27.0 * t9789 * t1652 - 4.0 / 9.0 * t9792 * t507 - 2.0 / 9.0 * t3727 * t1655 - t19863);
    (t23971, t23990)
}
