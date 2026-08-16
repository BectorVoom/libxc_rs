//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 978/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk978(t2106: f64, t8282: f64, t1775: f64, t9211: f64, t9230: f64, t11755: f64, t11761: f64, t12796: f64, t17338: f64, t1985: f64, t1986: f64, t2: f64, t2075: f64, t2097: f64, t2112: f64, t24: f64, t38556: f64, t38562: f64, t38572: f64, t38588: f64, t39769: f64, t40323: f64, t40327: f64, t40335: f64, t40337: f64, t462: f64, t558: f64, t582: f64, t9007: f64, t9016: f64, t92: f64) -> f64 {
    let t40357 = t8282 * t2106;
    let t40359 = t1775 * t9211;
    let t40361 = t1775 * t9230;
    let t40367 = 8.0_f64 / 3.0_f64 * t11755 * t12796 * t40323 - 8.0_f64 * t11761 * t17338 * t40327 * t558 + 8.0_f64 * t462 * t582 * t38562 + 112.0_f64 / 81.0_f64 * t40335 - 80.0_f64 / 81.0_f64 * t462 * t40337 * t38572 - 36.0_f64 * t462 * t9016 * t2 * t1986 * t2075 - 8.0_f64 * t462 * t2097 * t38588 - 2.0_f64 / 3.0_f64 * t462 * t2097 * t38556 + 8.0_f64 * t462 * t1985 * t2 * t9007 * t558 + 16.0_f64 / 9.0_f64 * t40357 + 8.0_f64 / 3.0_f64 * t40359 - 8.0_f64 * t40361 + 6.0_f64 * t92 * t24 * t2112 * t39769;
    t40367
}
