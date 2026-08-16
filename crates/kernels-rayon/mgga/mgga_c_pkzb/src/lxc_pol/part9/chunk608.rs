//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 608/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk608(t1020: f64, t614: f64, t568: f64, t2575: f64, t596: f64, t1029: f64, t1031: f64, t160: f64, t162: f64, t2625: f64, t2631: f64, t594: f64, t597: f64) -> (f64, f64, f64, f64) {
    let t2632 = t614 * t1020;
    let t2633 = t2632 * t568;
    let t2636 = t596 * t2575;
    let t2639 = 3.0_f64 * t1029 * t597 + 3.0_f64 * t1031 * t594 + 3.0_f64 * t160 * t2636 - t162 * t2625 - 12.0_f64 * t2631 * t2633;
    (t2632, t2633, t2636, t2639)
}
