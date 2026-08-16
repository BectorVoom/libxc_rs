//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1415/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1415(t169: f64, t1881: f64, t3712: f64, t1640: f64, t5407: f64, t446: f64, t4505: f64, t2132: f64, t3708: f64, t3709: f64, t18376: f64, t234: f64, t441: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t170 = t169 <= zeta_threshold;
    let t18388 = t1881 * t3712;
    let t18390 = t5407 * t1640;
    let t18391 = t446 * t18390;
    let t18393 = t1881 * t4505;
    let t18395 = t3708 * t2132;
    let t18396 = t446 * t18395;
    let t18398 = t1881 * t3709;
    let t18401 = piecewise3(t170, 0.0_f64, -t18376);
    let t18402 = t234 * t18401;
    let t18403 = t18402 * t441;
    (t18388, t18391, t18393, t18396, t18398, t18403)
}
