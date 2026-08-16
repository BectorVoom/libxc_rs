//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1449/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1449<F: Float>(t1220: F, t15012: F, t15016: F, t1579: F, t17440: F, t28109: F, t3284: F, t4536: F, t47709: F, t5233: F, t53911: F, t53914: F, t55816: F, t58399: F, t59220: F, t59431: F, t914: F) -> F {
    let t60292 = -t59220 + F::cast_from(400.0_f64) / F::cast_from(81.0_f64) * t55816 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t47709 - t28109 + t59431 - F::cast_from(176.0_f64) / F::cast_from(9.0_f64) * t15016 * t5233 - F::cast_from(2.0_f64) * t15012 * t5233 + F::cast_from(176.0_f64) / F::cast_from(9.0_f64) * t53911 * t1579 + F::cast_from(4.0_f64) * t4536 * t17440 + F::cast_from(8.0_f64) * t1220 * t914 * t3284 * t58399 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t53914 * t1579;
    t60292
}
