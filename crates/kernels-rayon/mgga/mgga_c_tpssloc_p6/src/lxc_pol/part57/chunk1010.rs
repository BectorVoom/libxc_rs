//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1010/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1010(t5: f64, t128333: f64, t128368: f64, t112: f64, t33610: f64, t7685: f64, t28813: f64, t8607: f64, t27188: f64, t7468: f64, t33234: f64, t28045: f64, t7042: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t128370 = piecewise3(t8, 0.0_f64, t128333 + t128368);
    let t128371 = t128370 * t112;
    let t128375 = 2.0_f64 * t7685 * t33610;
    let t128377 = 2.0_f64 * t8607 * t28813;
    let t128381 = 4.0_f64 * t27188 * t7468;
    let t128383 = 4.0_f64 * t33234 * t7468;
    let t128385 = 4.0_f64 * t7042 * t28045;
    (t128371, t128375, t128377, t128381, t128383, t128385)
}
