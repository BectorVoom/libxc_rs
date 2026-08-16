//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1309/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1309(t21846: f64, t4160: f64, t94425: f64, t17730: f64, t1889: f64, t6159: f64, t11814: f64, t29274: f64, t1394: f64, t8164: f64, t98618: f64, t28331: f64, t28499: f64, t5780: f64) -> (f64, f64, f64, f64, f64) {
    let t102431 = t4160 * t94425 * t21846;
    let t102438 = t6159 * t17730 * t1889;
    let t102441 = t11814 * t29274;
    let t102444 = t1394 * t98618 * t8164;
    let t102447 = t5780 * t28499 * t28331;
    (t102431, t102438, t102441, t102444, t102447)
}
