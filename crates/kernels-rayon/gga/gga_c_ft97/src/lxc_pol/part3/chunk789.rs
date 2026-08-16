//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 789/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk789(t16241: f64, t1903: f64, t1902: f64, t4545: f64, t487: f64, t492: f64, t83: f64, t4551: f64, t8466: f64, t1882: f64, t4617: f64, t3238: f64, t3271: f64, t452: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16242 = t1903 * t16241;
    let t16243 = t1902 * t16242;
    let t16246 = t4545 * t487;
    let t16247 = t16246 * t492;
    let t16248 = t83 * t16247;
    let t16251 = t8466 * t4551;
    let t16252 = t83 * t16251;
    let t16255 = t1882 * t4617;
    let t16258 = t452 * t3238 * t3271;
    (t16243, t16247, t16248, t16251, t16252, t16255, t16258)
}
