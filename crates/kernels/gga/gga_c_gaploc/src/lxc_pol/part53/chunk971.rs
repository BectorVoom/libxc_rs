//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 971/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk971<F: Float>(t13870: F, t795: F, t313: F, t2639: F, t13857: F, t4614: F, t813: F, t1: F, t106: F, t316: F, t780: F, t13858: F, t2194: F) -> (F, F, F, F, F) {
    let t47326 = t795 * t13870;
    let t47327 = t313 * t47326;
    let t47329 = F::new(0.10725146985555128001e1) * t47327 * t2639;
    let t47331 = t813 * t4614 * t13857;
    let t47338 = t13870 * t1 * t106 * t316;
    let t47340 = F::new(0.35750489951850426669e0) * t780 * t47338;
    let t47341 = t2194 * t13858;
    (t47326, t47329, t47331, t47340, t47341)
}
