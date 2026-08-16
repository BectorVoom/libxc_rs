//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1049/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1049(t3845: f64, t884: f64, t1437: f64, t2569: f64, t2551: f64, t3848: f64, t2577: f64, t3844: f64, t11245: f64, t11248: f64, t11251: f64, t11255: f64, t11258: f64, t11262: f64, t11265: f64, t2550: f64, t2575: f64, t3827: f64, t3849: f64, t8842: f64, t8847: f64, t8899: f64) -> f64 {
    let t11418 = t3845 * t884;
    let t11421 = t1437 * t2569;
    let t11424 = t3848 * t2551;
    let t11427 = t3844 * t2577;
    let t11428 = t11427 * t884;
    let t11431 = t11245 + t11248 + t11251 - t11255 - t11258 - t11262 - t11265 - 4.0_f64 * t8899 * t3827 + 0.64327917994770140268e2_f64 * t8842 * t3849 - 4.0_f64 * t2550 * t11418 - 2.0_f64 * t2550 * t11421 - 0.19298375398431042081e3_f64 * t8847 * t11424 + 0.64327917994770140268e2_f64 * t2575 * t11428;
    t11431
}
