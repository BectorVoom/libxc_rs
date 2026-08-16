//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2000/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2000(t23171: f64, t23228: f64, t6572: f64, t212: f64, t6554: f64, t852: f64, t23030: f64, t23253: f64, t6555: f64, t81573: f64, t6563: f64, t81597: f64) -> (f64, f64, f64, f64, f64) {
    let t82082 = t23171 * t23228 * t6572;
    let t82087 = t23171 * t212 * t852 * t6554;
    let t82099 = t23030 * t23253;
    let t82120 = t81573 * t23228 * t6555;
    let t82122 = t81597 * t6563;
    (t82082, t82087, t82099, t82120, t82122)
}
