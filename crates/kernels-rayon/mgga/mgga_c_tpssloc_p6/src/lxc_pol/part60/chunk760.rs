//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 760/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk760(t27971: f64, t72: f64, t5445: f64, t79: f64, t5392: f64, t605: f64, t5399: f64, t1873: f64, t19451: f64, t1441: f64, t1458: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27972 = t72 * t27971;
    let t27975 = t79 * t5445;
    let t27976 = t72 * t27975;
    let t27979 = t605 * t5392;
    let t27982 = t605 * t5399;
    let t28001 = 2.0_f64 * t19451 * t1873;
    let t28002 = t1441 * t1458;
    (t27972, t27976, t27979, t27982, t28001, t28002)
}
