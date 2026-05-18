//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1046/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1046<F: Float>(t11269: F, t1526: F, t1527: F, t1528: F, t15567: F, t15568: F, t15575: F, t19972: F, t20022: F, t20031: F, t20039: F, t20044: F, t20107: F, t20130: F, t3088: F, t38327: F, t38355: F, t38357: F, t75878: F, t75881: F) -> F {
    let t86536 = t15567 * t15575 * t20039 / F::new(2.0) + t1526 * t1527 * t20107 / F::new(2.0) + F::new(2.0) / F::new(3.0) * t1526 * t3088 * t38357 * t20022 - t15567 * t15568 * t20031 / F::new(3.0) - t38355 - t1526 * t3088 * t20130 / F::new(3.0) - F::new(7.0) / F::new(27.0) * t1526 * t11269 * t38327 * t20022 + F::new(2.0) * t19972 - t75878 / F::new(6.0) - t75881 / F::new(9.0) - t1526 * t1527 * t1528 * t20044 / F::new(12.0);
    t86536
}
