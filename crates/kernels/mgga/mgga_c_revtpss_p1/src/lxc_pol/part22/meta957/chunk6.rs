//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3213/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3213<F: Float>(t13334: F, t13340: F, t13393: F, t13396: F, t13397: F, t13400: F, t13405: F, t1470: F, t1486: F, t1494: F, t21707: F, t21710: F, t21713: F, t2312: F, t38: F, t4181: F, t4182: F, t4187: F, t4217: F, t4238: F, t5830: F, t60937: F, t60987: F, t641: F, t85: F) -> F {
    let t60994 = -t13396 * t1486 * t85 / F::cast_from(3.0_f64) - t4181 * t4217 * t85 / F::cast_from(3.0_f64) - t21707 * t641 / F::cast_from(3.0_f64) - t13405 * t1486 * t85 / F::cast_from(6.0_f64) - t4187 * t4217 * t85 / F::cast_from(3.0_f64) - t21710 * t641 / F::cast_from(3.0_f64) - t1470 * t13334 * t85 / F::cast_from(6.0_f64) - t21713 * t641 / F::cast_from(3.0_f64) - t5830 * t2312 / F::cast_from(6.0_f64) - t13393 * t1494 / F::cast_from(6.0_f64) - t13397 * t1494 / F::cast_from(3.0_f64) - t13400 * t1494 / F::cast_from(3.0_f64) - t4182 * t4238 / F::cast_from(3.0_f64) + t38 * (t60937 + t60987) * t85 / F::cast_from(24.0_f64) - t13340 * t1494 / F::cast_from(6.0_f64);
    t60994
}
