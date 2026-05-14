//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1010/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1010<F: Float>(t1281: F, t204: F, t2739: F, t1878: F, t218: F, t2774: F, t2778: F, t1079: F, t5555: F, t1107: F, t5838: F, t1854: F, t2743: F, t1070: F, t5801: F, t1095: F, t1938: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t20716 = t204 * t1281 * t2739;
    let t20717 = 0.12077e1 * t20716;
    let t20748 = t218 * t1878 * t2774;
    let t20749 = 0.82785e0 * t20748;
    let t20751 = t218 * t1878 * t2778;
    let t20752 = 0.82785e0 * t20751;
    let t20754 = t218 * t5555 * t1079;
    let t20787 = 4.0 / 3.0 * t20716;
    let t20834 = t5838 * t1107;
    let t20845 = 0.37083333333333333334e-1 * t20716;
    let t20861 = 0.11958666666666666667e1 * t20716;
    let t20893 = t2743 * t1854;
    let t20896 = t1070 * t5801;
    let t20905 = t1938 * t1095;
    (t20716, t20717, t20748, t20749, t20751, t20752, t20754, t20787, t20834, t20845, t20861, t20893, t20896, t20905)
}
