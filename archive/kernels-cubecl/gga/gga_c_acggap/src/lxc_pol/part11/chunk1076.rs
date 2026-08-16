//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1076/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1076<F: Float>(t1165: F, t19834: F, t2068: F, t604: F, t1083: F, t1487: F, t1980: F, t355: F, t7458: F, t2118: F, t4999: F, t7799: F, t8571: F) -> (F, F, F, F) {
    let t34762 = t2068 * t1165 * t604 * t19834;
    let t34767 = t1980 * t7458 * t1083 * t355 * t1487;
    let t34769 = t2118 * t4999;
    let t34771 = t7799 * t8571;
    (t34762, t34767, t34769, t34771)
}
