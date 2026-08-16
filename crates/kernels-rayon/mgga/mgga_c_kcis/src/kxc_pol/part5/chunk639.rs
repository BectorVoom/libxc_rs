//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 639/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk639(t1444: f64, t617: f64, t1606: f64, t616: f64, t494: f64, t1625: f64, t1628: f64, t1627: f64, t632: f64, t629: f64, t1646: f64, t2629: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4445 = t617 * t1444;
    let t4455 = 1.0_f64 / t1606 / t616;
    let t4456 = t494 * t4455;
    let t4475 = t1625 * t1628;
    let t4479 = 1.0_f64 / t1627 / t632;
    let t4480 = t629 * t4479;
    let t4510 = t2629 * t1646;
    (t4445, t4455, t4456, t4475, t4479, t4480, t4510)
}
