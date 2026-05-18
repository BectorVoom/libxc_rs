//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 706/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk706<F: Float>(t20371: F, t20387: F, t457: F, t91: F, t11076: F, t20105: F, t20109: F, t20119: F, t20126: F, t20132: F, t20139: F, t20143: F, t20147: F, t20154: F, t20331: F, t8260: F) -> (F, F, F) {
    let t20388 = t20371 + t20387;
    let t20390 = t91 * t457 * t20388;
    let t20394 = t20147 + F::new(2.0) * t20154 + F::new(2.0) / F::new(3.0) * t20132 - F::new(2.0) / F::new(3.0) * t20139 + t20143 - F::new(2.0) * t20105 - F::new(2.0) * t20109 - F::new(4.0) / F::new(3.0) * t11076 - t8260 + F::new(3.0) / F::new(8.0) * t20331 + t20390 / F::new(2.0) + F::new(6.0) * t20119 - F::new(10.0) / F::new(27.0) * t20126;
    (t20388, t20390, t20394)
}
