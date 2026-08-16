//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1080/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1080(t7784: f64, t8083: f64, t20572: f64, t27028: f64, t5329: f64, t1267: f64, t1851: f64, t26996: f64, t251: f64, t5345: f64, t1250: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28176 = t8083 * t7784;
    let t28178 = t27028 * t20572;
    let t28179 = t5329 * t28178;
    let t28182 = t1851 * t1267;
    let t28183 = t26996 * t28182;
    let t28184 = t5329 * t28183;
    let t28189 = t5345 * t251;
    let t28190 = t28189 * t1250;
    (t28176, t28178, t28179, t28182, t28183, t28184, t28189, t28190)
}
