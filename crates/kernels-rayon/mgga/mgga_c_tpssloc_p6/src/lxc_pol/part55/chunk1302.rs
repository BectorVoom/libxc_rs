//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1302/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1302(t118345: f64, t118347: f64, t125982: f64, t125988: f64, t125991: f64, t126004: f64, t126015: f64, t1396: f64, t1398: f64, t1852: f64, t2170: f64, t27930: f64, t32649: f64, t34401: f64, t5364: f64, t7416: f64, t7426: f64, t8111: f64, t8119: f64, t8927: f64) -> f64 {
    let t126018 = 2.0_f64 * t8111 * t7426 + 2.0_f64 * t125982 + 2.0_f64 * t7416 * t8119 + 2.0_f64 * t2170 * t27930 + t118345 + t118347 + t125988 + t5364 * t8927 + t1852 * t32649 + t125991 + t1396 * t34401 + t1398 * (t126004 + t126015);
    t126018
}
