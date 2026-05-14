//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 558/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk558<F: Float>(t1705: F, t1791: F, t1852: F, t2038: F, t607: F, t759: F, t761: F, t1393: F, t1396: F, t1870: F) -> (F, F, F) {
    let t2040 = t1705 + t1791 + t1852 + t2038;
    let t2045 = t759 * t607 * t761;
    let t2049 = -0.49388888888888888889e-2 * t1393 + 0.98777777777777777777e-2 * t1396 + t1870;
    (t2040, t2045, t2049)
}
