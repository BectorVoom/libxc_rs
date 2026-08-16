//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1141/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1141(t4972: f64, t7709: f64, t5329: f64, t1094: f64, t1748: f64, t1122: f64, t303: f64, t1134: f64, t1749: f64, t1768: f64, t26796: f64, t1250: f64, t14570: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27918 = t7709 * t4972;
    let t27919 = t5329 * t27918;
    let t27924 = t1748 * t1094;
    let t27925 = t27924 * t1122;
    let t27926 = t303 * t27925;
    let t27928 = t1749 * t1134;
    let t27929 = t303 * t27928;
    let t27931 = t26796 * t1768;
    let t27932 = t303 * t27931;
    let t27936 = t14570 * t1250;
    (t27918, t27919, t27924, t27925, t27926, t27928, t27929, t27931, t27932, t27936)
}
