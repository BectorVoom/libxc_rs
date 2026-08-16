//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2022/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2022(t2470: f64, t28359: f64, t7064: f64, t7997: f64, t822: f64, t28313: f64, t25387: f64, t95822: f64, t98892: f64, t95537: f64, t1957: f64, t26550: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t103421 = t28359 * t2470;
    let t103422 = t7064 * t103421;
    let t103424 = t822 * t7997;
    let t103431 = t28313 * t2470;
    let t103432 = t25387 * t103431;
    let t103435 = 0.28912093960683998208e-1_f64 * t95822 * t98892;
    let t103437 = 0.51405703062096148812e-1_f64 * t95537 * t98892;
    let t103438 = t1957 * t26550;
    (t103421, t103422, t103424, t103431, t103432, t103435, t103437, t103438)
}
