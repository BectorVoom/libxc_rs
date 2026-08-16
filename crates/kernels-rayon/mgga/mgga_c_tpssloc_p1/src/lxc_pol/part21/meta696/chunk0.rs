//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2523/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2523(t14165: f64, t43070: f64, t10190: f64, t13835: f64, t2986: f64, t42841: f64, t10254: f64, t12652: f64, t1597: f64, t43052: f64, t2990: f64, t10255: f64, t13847: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47927 = t43070 * t14165;
    let t47938 = t2986 * t10190 * t13835;
    let t47941 = t42841 * t14165;
    let t47966 = t10254 * t12652;
    let t48019 = t43052 * t1597;
    let t48021 = t2986 * t48019 * t2990;
    let t48024 = t2986 * t13847 * t10255;
    (t47927, t47938, t47941, t47966, t48019, t48021, t48024)
}
