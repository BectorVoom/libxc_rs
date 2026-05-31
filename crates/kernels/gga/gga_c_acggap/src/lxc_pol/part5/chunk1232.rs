//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1232/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1232<F: Float>(t13804: F, t13805: F, t16209: F, t16211: F, t16213: F, t21669: F, t21671: F, t21675: F, t21679: F, t21681: F, t21684: F, t21687: F, t21691: F, t21695: F) -> F {
    let t22563 = F::cast_from(4.0_f64) * t21669 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t21671 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t21675 + t21679 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t21681 + t21684 + t21687 / F::cast_from(2.0_f64) + F::cast_from(6.0_f64) * t21691 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t21695 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t16209 - F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t16211 + F::cast_from(140.0_f64) / F::cast_from(27.0_f64) * t16213 - t13804 + t13805;
    t22563
}
