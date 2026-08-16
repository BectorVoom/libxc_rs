//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1103/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1103(t1281: f64, t204: f64, t2739: f64, t1878: f64, t218: f64, t2774: f64, t2778: f64, t1079: f64, t5555: f64, t1107: f64, t5838: f64, t1854: f64, t2743: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20716 = t204 * t1281 * t2739;
    let t20717 = 0.12077e1_f64 * t20716;
    let t20748 = t218 * t1878 * t2774;
    let t20749 = 0.82785e0_f64 * t20748;
    let t20751 = t218 * t1878 * t2778;
    let t20752 = 0.82785e0_f64 * t20751;
    let t20754 = t218 * t5555 * t1079;
    let t20787 = 4.0_f64 / 3.0_f64 * t20716;
    let t20834 = t5838 * t1107;
    let t20845 = 0.37083333333333333334e-1_f64 * t20716;
    let t20861 = 0.11958666666666666667e1_f64 * t20716;
    let t20893 = t2743 * t1854;
    (t20716, t20717, t20748, t20749, t20751, t20752, t20754, t20787, t20834, t20845, t20861, t20893)
}
