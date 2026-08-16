//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 971/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk971(t13870: f64, t795: f64, t313: f64, t2639: f64, t13857: f64, t4614: f64, t813: f64, t1: f64, t106: f64, t316: f64, t780: f64, t13858: f64, t2194: f64) -> (f64, f64, f64, f64, f64) {
    let t47326 = t795 * t13870;
    let t47327 = t313 * t47326;
    let t47329 = 0.10725146985555128001e1_f64 * t47327 * t2639;
    let t47331 = t813 * t4614 * t13857;
    let t47338 = t13870 * t1 * t106 * t316;
    let t47340 = 0.35750489951850426669e0_f64 * t780 * t47338;
    let t47341 = t2194 * t13858;
    (t47326, t47329, t47331, t47340, t47341)
}
