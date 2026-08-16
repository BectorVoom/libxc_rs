//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1317/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1317(t33211: f64, t6535: f64, t191: f64, t192: f64, t26138: f64, t2020: f64, t33137: f64, t6876: f64, t22574: f64, t25988: f64, t36533: f64, t25985: f64, t8450: f64) -> (f64, f64, f64, f64, f64) {
    let t120069 = 4.0_f64 * t33211 * t6535;
    let t120071 = t26138 * t191 * t192;
    let t120072 = t120071 * t2020;
    let t120075 = 2.0_f64 * t6876 * t33137;
    let t120078 = 6.0_f64 * t22574 * t36533 * t25988;
    let t120079 = t8450 * t25985;
    (t120069, t120072, t120075, t120078, t120079)
}
