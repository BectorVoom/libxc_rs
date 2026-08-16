//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1164/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1164(t23273: f64, t81591: f64, t23228: f64, t6555: f64, t81573: f64, t6563: f64, t81597: f64, t214: f64, t2710: f64, t1880: f64, t6572: f64, t23196: f64, t23237: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82115 = t81591 * t23273;
    let t82120 = t81573 * t23228 * t6555;
    let t82122 = t81597 * t6563;
    let t82124 = t214 * t2710;
    let t82126 = t1880 * t82124 * t6572;
    let t82129 = t1880 * t23237 * t23196;
    (t82115, t82120, t82122, t82124, t82126, t82129)
}
