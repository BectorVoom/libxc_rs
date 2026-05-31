//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1045/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1045<F: Float>(t1526: F, t1527: F, t19970: F, t20022: F, t20098: F, t20141: F, t20145: F, t20163: F, t342: F, t343: F, t61180: F, t61184: F, t72: F, t75935: F, t75944: F, t75947: F, t7712: F) -> F {
    let t86508 = t19970 - t75944 / F::cast_from(12.0_f64) + t75947 / F::cast_from(6.0_f64) + t20163 - t342 * t343 * t72 * t20098 / F::cast_from(4.0_f64) - t75935 / F::cast_from(4.0_f64) + t61180 / F::cast_from(6.0_f64) + t61184 / F::cast_from(18.0_f64) - t1526 * t1527 * t20145 / F::cast_from(4.0_f64) - t1526 * t1527 * t20141 / F::cast_from(4.0_f64) - t1526 * t1527 * t7712 * t20022 / F::cast_from(2.0_f64);
    t86508
}
