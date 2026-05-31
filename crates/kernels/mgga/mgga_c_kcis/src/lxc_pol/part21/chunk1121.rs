//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1121/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1121<F: Float>(t2201: F, t3668: F, t2205: F, t3699: F, t26867: F, t26870: F, t26873: F, t26874: F, t26875: F, t26879: F, t26882: F, t26888: F, t3669: F, t3670: F, t7809: F) -> (F, F, F) {
    let t27141 = t2201 * t3668;
    let t27144 = t2205 * t3699;
    let t27147 = F::cast_from(2.0_f64) * t27141 * t3670 + F::cast_from(2.0_f64) * t27144 * t3669 - t3699 * t7809 - t26867 + t26870 - t26873 + t26874 + t26875 + t26879 + t26882 - t26888;
    (t27141, t27144, t27147)
}
