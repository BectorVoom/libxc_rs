//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1411/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1411<F: Float>(t1220: F, t2367: F, t8421: F, t3277: F, t8410: F, t8416: F, t11786: F, t11894: F, t26488: F, t26490: F, t26493: F, t26560: F, t26853: F, t27346: F, t3281: F, t3286: F, t4281: F, t4289: F, t9241: F) -> F {
    let t28066 = t1220 * t2367 * t8421;
    let t28068 = t8410 * t3277;
    let t28071 = t1220 * t2367 * t8416;
    let t28082 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t28066 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t28068 + t26488 + t26490 + t26493 - t26560 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t28071 + t8410 * t3281 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t8410 * t3286 - F::cast_from(4.0_f64) * t11786 * t9241 + t26853 - F::cast_from(8.0_f64) * t4281 * t4289 * t11894 * t27346;
    t28082
}
