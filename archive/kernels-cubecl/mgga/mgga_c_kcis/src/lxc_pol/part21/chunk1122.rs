//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1122/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1122<F: Float>(t187: F, t26867: F, t26870: F, t26873: F, t26874: F, t26875: F, t26877: F, t26879: F, t26882: F, t26885: F, t26888: F, t26951: F, t27139: F, t27147: F) -> F {
    let t27150 = t26867 - t26870 + t26873 - t26874 - t26875 + t26877 - t26879 - t26882 + t26885 + t26888 - t26951 + t187 * (t27139 + t27147);
    t27150
}
