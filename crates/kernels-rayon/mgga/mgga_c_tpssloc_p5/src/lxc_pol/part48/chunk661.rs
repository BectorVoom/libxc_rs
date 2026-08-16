//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 661/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk661(t5: f64, t63: f64, t8307: f64, t8513: f64, t8663: f64, t112: f64, t2039: f64, t2165: f64) -> (f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t8824 = t8307 * t63;
    let t8825 = t8513 * t8824;
    let t8828 = piecewise3(t8, 0.0_f64, -5.0_f64 / 72.0_f64 * t8663 * t8825);
    let t8829 = t8828 * t112;
    let t8835 = t2165 * t2039;
    (t8824, t8825, t8828, t8829, t8835)
}
