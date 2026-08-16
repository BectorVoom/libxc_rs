//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1247/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1247(t21233: f64, t21236: f64, t21239: f64, t21251: f64, t21255: f64, t21257: f64, t21320: f64, t21324: f64, t21329: f64, t21331: f64, t21333: f64, t21186: f64, t21196: f64, t21217: f64, t21220: f64, t21223: f64, t21225: f64, t21308: f64, t21313: f64, t21315: f64, t21318: f64, t21814: f64, t21815: f64, t21817: f64) -> f64 {
    let t21819 = -t21320 + t21233 + t21236 + t21239 - t21324 - t21329 - t21331 - t21333 - t21251 + t21255 + t21257;
    let t21822 = t21814 + t21815 + t21817 - t21308 + t21313 - t21315 - t21318 + t21186 - t21196 + t21217 + t21220 + t21223 + t21225 + t21819;
    t21822
}
