//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 374/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk374(t1795: f64, t336: f64, t368: f64, t1501: f64, t495: f64, t1734: f64, t337: f64, t513: f64, t535: f64, t1150: f64, t1180: f64, t127: f64, t1353: f64, t1355: f64, t1424: f64, t1435: f64, t1761: f64, t1775: f64, t1784: f64, t1788: f64, t335: f64, t367: f64) -> (f64, f64, f64, f64, f64) {
    let t1797 = t336 * t368 * t1795;
    let t1801 = t336 * t1501 * t495;
    let t1805 = t336 * t337 * t1734;
    let t1809 = t336 * t535 * t513;
    let t1812 = 0.85748036236139473944e-3_f64 * t1180 * t1761 + t127 * t1775 / 96.0_f64 - 0.85748036236139473944e-3_f64 * t1353 - 0.42874018118069736972e-3_f64 * t1355 + 0.17149607247227894789e-2_f64 * t1424 + 0.85748036236139473944e-3_f64 * t1435 + t367 * t1784 / 48.0_f64 + t1150 * t1788 / 16.0_f64 - t367 * t1797 / 96.0_f64 - t335 * t1801 / 24.0_f64 - t335 * t1805 / 48.0_f64 - t367 * t1809 / 48.0_f64;
    (t1797, t1801, t1805, t1809, t1812)
}
