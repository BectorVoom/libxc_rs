//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 292/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk292(t1475: f64, t446: f64, t221: f64, t1439: f64, t205: f64, t206: f64, t23: f64, t1156: f64, t589: f64, t1392: f64, t472: f64, t207: f64, t470: f64, t473: f64, t600: f64, t602: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1476 = t1475 * t446;
    let t1477 = t221 * t1476;
    let t1480 = t1439 * t205;
    let t1486 = t206 * t23;
    let t1487 = t1156 * t589;
    let t1488 = t1487 * t446;
    let t1491 = t472 * t1392;
    let t1494 = -t1480 * t207 - 12.0_f64 * t1486 * t1488 + 3.0_f64 * t1491 * t206 + 3.0_f64 * t470 * t602 + 3.0_f64 * t473 * t600;
    (t1477, t1480, t1486, t1487, t1488, t1491, t1494)
}
