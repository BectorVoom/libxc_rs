//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 745/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk745<F: Float>(t16925: F, t16928: F, t20543: F, t20547: F, t20554: F, t20558: F, t20562: F, t20566: F, t20570: F, t20658: F, t20663: F, t20784: F, t20839: F) -> F {
    let t20971 = F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t20784 + t20839 / F::cast_from(2.0_f64) + t16925 - F::cast_from(2.0_f64) * t16928 - t20658 - F::cast_from(6.0_f64) * t20663 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t20554 + t20558 + t20562 - F::cast_from(2.0_f64) * t20566 - F::cast_from(2.0_f64) * t20570 + F::cast_from(2.0_f64) * t20543 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t20547;
    t20971
}
