//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 641/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk641(t26050: f64, t83: f64, t1882: f64, t6531: f64, t6540: f64, t3214: f64, t452: f64, t5710: f64, t11593: f64, t1901: f64, t23152: f64, t26185: f64, t26189: f64, t26192: f64, t26195: f64, t26199: f64, t26203: f64, t26207: f64, t26211: f64, t26214: f64, t446: f64) -> f64 {
    let t26217 = t83 * t26050;
    let t26220 = t1882 * t6531;
    let t26222 = t1882 * t6540;
    let t26225 = t452 * t5710 * t3214;
    let t26228 = t1901 * t26185 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t11593 * t26189 + t26192 / 9.0_f64 - t446 * t26195 / 3.0_f64 - t1901 * t26199 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t26203 - t446 * t26207 / 9.0_f64 + t23152 + 2.0_f64 / 9.0_f64 * t11593 * t26211 + t1901 * t26214 / 9.0_f64 - t446 * t26217 / 3.0_f64 + t26220 / 9.0_f64 - t26222 / 9.0_f64 + t446 * t26225 / 3.0_f64;
    t26228
}
