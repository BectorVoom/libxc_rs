//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 436/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk436(t200: f64, t2417: f64, t680: f64, t2379: f64, t2395: f64, t235: f64, t693: f64, t226: f64, t709: f64, t209: f64, t625: f64, t228: f64, t231: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2418 = t2417 * t200;
    let t2419 = t680 * t2418;
    let t2422 = t2379 * t2395;
    let t2426 = 1.0_f64 / t693 / t235;
    let t2427 = t226 * t2426;
    let t2428 = t709 * t709;
    let t2429 = t2427 * t2428;
    let t2432 = t209 * t625;
    let t2434 = t228 * t2432 * t231;
    (t2419, t2422, t2426, t2427, t2428, t2429, t2434)
}
