//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1209/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1209(t2047: f64, t9971: f64, t2627: f64, t7084: f64, t24251: f64, t24270: f64, t24273: f64, t2617: f64, t2633: f64, t4182: f64, t4281: f64, t7101: f64, t7102: f64, t812: f64, t81617: f64, t81623: f64, t81627: f64, t81630: f64, t81633: f64, t81637: f64, t81642: f64, t81645: f64, t81648: f64, t81653: f64, t84842: f64, t9612: f64, t9661: f64, t9976: f64) -> f64 {
    let t84953 = t9971 * t2047;
    let t84962 = t2627 * t7084;
    let t84981 = -0.11514538467937585055e0_f64 * t81617 - 3.0_f64 * t9612 * t7102 - 6.0_f64 * t812 * t84953 * t9976 - 3.0_f64 * t2617 * t24273 + 6.0_f64 * t4281 * t84842 * t4182 + 6.0_f64 * t812 * t84962 * t2633 + 0.46058153871750340221e0_f64 * t81623 - 0.3289868133696452873e-1_f64 * t81627 + 0.49348022005446793095e-1_f64 * t81630 - 0.76763589786250567036e0_f64 * t81633 - 0.9869604401089358619e-1_f64 * t81637 - 0.14804406601634037928e0_f64 * t81642 - 6.0_f64 * t2617 * t24270 - t812 * t7101 * t9661 + 0.9869604401089358619e-1_f64 * t81645 - 0.49348022005446793095e-1_f64 * t81648 - 0.9869604401089358619e-1_f64 * t81653 - 3.0_f64 * t2617 * t24251;
    t84981
}
