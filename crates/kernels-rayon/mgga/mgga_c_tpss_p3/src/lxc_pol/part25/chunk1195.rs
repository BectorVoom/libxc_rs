//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1195/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1195(t1232: f64, t1656: f64, t520: f64, t1265: f64, t1640: f64, t1258: f64, t5570: f64, t1771: f64, t12828: f64, t12823: f64, t196: f64, t197: f64, t4352: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19521 = t1656 * t1232 * t520;
    let t19535 = t1640 * t1265;
    let t19539 = t5570 * t1258;
    let t19540 = t1771 * t19539;
    let t19542 = t12828 * t1232;
    let t19554 = t12823 * t520;
    let t19577 = t4352 * t196 * t197;
    (t19521, t19535, t19539, t19540, t19542, t19554, t19577)
}
