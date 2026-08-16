//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1484/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1484(t11913: f64, t44690: f64, t11881: f64, t11883: f64, t11884: f64, t11897: f64, t11916: f64, t1244: f64, t1246: f64, t3604: f64, t3610: f64, t3612: f64, t3613: f64, t3626: f64, t44669: f64, t44673: f64, t44700: f64, t44710: f64, t44785: f64, t44786: f64, t44798: f64, t45314: f64, t45320: f64, t45323: f64, t45326: f64, t470: f64, t491: f64, t493: f64) -> f64 {
    let t45329 = t44690 * t11913;
    let t45332 = t1244 * t1246 * t44798 * t491 + 24.0_f64 * t11881 * t11883 * t44673 + 6.0_f64 * t3610 * t3612 * t44669 + 12.0_f64 * t3610 * t3612 * t44710 - t44700 * t44785 * t44786 + t45314 * t470 * t493 + 24.0_f64 * t11884 * t45326 + 12.0_f64 * t11897 * t3604 + 4.0_f64 * t11916 * t45329 + 12.0_f64 * t3613 * t45320 - 6.0_f64 * t3626 * t45323;
    t45332
}
