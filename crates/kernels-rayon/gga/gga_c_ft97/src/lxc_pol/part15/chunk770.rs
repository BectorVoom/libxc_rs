//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 770/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk770(t1096: f64, t5049: f64, t680: f64, t1613: f64, t236: f64, t679: f64, t3771: f64, t39: f64, t1689: f64, t5009: f64, t5014: f64, t4960: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21325 = t680 * t1096 * t5049;
    let t21328 = t236 * t1613;
    let t21329 = t21328 * t679;
    let t21330 = t3771 * t21329;
    let t21331 = t1096 * t39;
    let t21332 = t1689 * t5009;
    let t21333 = t21332 * t5014;
    let t21337 = t4960 * t6;
    (t21325, t21328, t21329, t21330, t21331, t21332, t21333, t21337)
}
