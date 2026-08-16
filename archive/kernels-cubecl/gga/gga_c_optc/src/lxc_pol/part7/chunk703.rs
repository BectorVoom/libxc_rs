//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 703/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk703<F: Float>(t6693: F, t2048: F, t592: F, t188: F, t1912: F, t1956: F, t6647: F, t6648: F, t6675: F, t6682: F, t6684: F, t6687: F, t6689: F, t737: F) -> (F, F, F) {
    let t6694 = F::cast_from(96.0_f64) * t6693;
    let t6695 = t2048 * t592;
    let t6696 = F::cast_from(96.0_f64) * t6695;
    let t6697 = -t6647 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t6648 + t188 * t6675 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t737 * t1956 + F::cast_from(35.0_f64) / F::cast_from(3.0_f64) * t6682 - F::cast_from(7.0_f64) * t6684 - F::cast_from(7.0_f64) / F::cast_from(2.0_f64) * t6687 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t6689 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t737 * t1912 - t6694 - t6696;
    (t6694, t6696, t6697)
}
