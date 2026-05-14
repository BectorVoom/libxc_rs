//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 727/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk727<F: Float>(t496: F, t8428: F, t1587: F, t429: F, t7205: F, t1585: F, t1443: F, t1773: F) -> (F, F, F, F) {
    let t11597 = t496 * t8428;
    let t11603 = t7205 * t429 * t1587;
    let t11604 = t1585 * t11603;
    let t11671 = t1773 * t1443;
    (t11597, t11603, t11604, t11671)
}
