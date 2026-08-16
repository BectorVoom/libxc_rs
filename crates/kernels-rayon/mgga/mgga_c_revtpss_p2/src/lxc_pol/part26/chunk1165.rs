//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1165/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1165(t26375: f64, t531: f64, t530: f64, t7535: f64, t10263: f64, t1450: f64, t1453: f64, t2014: f64, t2106: f64, t2107: f64, t2108: f64, t2320: f64, t2322: f64, t25089: f64, t25177: f64, t25188: f64, t25802: f64, t25865: f64, t26154: f64, t26162: f64, t26376: f64, t26380: f64, t26411: f64, t26674: f64, t26699: f64, t46304: f64, t508: f64, t649: f64, t7235: f64, t7238: f64, t7315: f64, t7359: f64, t7474: f64, t7488: f64, t7489: f64, t7536: f64, t9400: f64, t94349: f64, t95002: f64, t95019: f64, t95371: f64) -> f64 {
    let t95464 = t531 * t26375;
    let t95472 = t530 * t7535;
    let t95499 = -3.0_f64 * t2320 * t7474 - t2014 * t2107 * t46304 - 3.0_f64 * t2014 * t7536 * t25802 + 6.0_f64 * t2014 * t9400 * t2106 * t1450 + 3.0_f64 * t26699 * t1453 - 6.0_f64 * t2014 * t2107 * t94349 + 9.0_f64 * t2014 * t95464 * t7238 - 3.0_f64 * t649 * t26674 - 6.0_f64 * t95371 * t508 + 18.0_f64 * t2014 * t95472 * t25865 + 9.0_f64 * t25188 * t7489 - 6.0_f64 * t7235 * t26380 + t95019 * t2108 + 6.0_f64 * t2014 * t7536 * t25177 + 3.0_f64 * t2014 * t7488 * t95002 - 6.0_f64 * t2322 * t26154 + 9.0_f64 * t2014 * t26411 * t25089 + 18.0_f64 * t7235 * t26162 - 6.0_f64 * t7359 * t10263 - 3.0_f64 * t2014 * t26376 * t7315;
    t95499
}
