//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 779/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk779(t1114: f64, t1501: f64, t3068: f64, t3090: f64, t242: f64, t1125: f64, t2840: f64, t3096: f64, t3426: f64, t3931: f64, t1127: f64, t2845: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4270 = t1501 * t1114;
    let t4271 = t3068 * t4270;
    let t4274 = t3090 * t1501;
    let t4275 = t242 * t4274;
    let t4276 = t1125 * t4275;
    let t4278 = t3096 * t2840;
    let t4279 = t4278 * t3426;
    let t4280 = t3931 * t4279;
    let t4283 = t1127 * t2845;
    (t4270, t4271, t4276, t4278, t4279, t4280, t4283)
}
