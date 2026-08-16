//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1313/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1313(t1232: f64, t43710: f64, t5381: f64, t1656: f64, t4459: f64, t520: f64, t5432: f64, t1639: f64, t4516: f64, t5448: f64, t1265: f64, t12828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t69663 = t43710 * t1232;
    let t69667 = t5381 * t1232;
    let t69676 = t1656 * t4459 * t520;
    let t69681 = t5432 * t1232 * t520;
    let t69691 = t4516 * t1639 * t520;
    let t69699 = t5448 * t1232 * t520;
    let t69704 = t5381 * t1265;
    let t69708 = t12828 * t4459;
    (t69663, t69667, t69676, t69681, t69691, t69699, t69704, t69708)
}
