//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1054/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1054(t2267: f64, t8233: f64, t2259: f64, t8252: f64, t30226: f64, t470: f64, t487: f64, t14365: f64, t2271: f64, t8283: f64, t499: f64, t498: f64) -> (f64, f64, f64, f64, f64) {
    let t31261 = t8233 * t2267;
    let t31263 = t2259 * t8252;
    let t31265 = t470 * t30226;
    let t31266 = t487 * t31265;
    let t31267 = t14365 * t31266;
    let t31269 = t2271 * t8283;
    let t31271 = t499 * t30226;
    let t31272 = t498 * t31271;
    (t31261, t31263, t31267, t31269, t31272)
}
