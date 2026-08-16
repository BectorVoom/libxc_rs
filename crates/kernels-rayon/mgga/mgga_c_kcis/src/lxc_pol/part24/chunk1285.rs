//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1285/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1285(t14443: f64, t29006: f64, t7703: f64, t1003: f64, t18482: f64, t26686: f64, t1008: f64, t27806: f64, t70767: f64, t13097: f64, t4977: f64, t13376: f64, t1704: f64) -> (f64, f64, f64, f64, f64) {
    let t101035 = t7703 * t14443 * t29006;
    let t101043 = t26686 * t18482 * t1003;
    let t101047 = t27806 * t70767 * t1008;
    let t101053 = t27806 * t13097 * t4977;
    let t101057 = t26686 * t13376 * t1704;
    (t101035, t101043, t101047, t101053, t101057)
}
