//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 887/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk887(t1036: f64, t1089: f64, t175: f64, t839: f64, t864: f64, t1103: f64, t3244: f64, t1005: f64, t3493: f64, t3292: f64, t3101: f64, t360: f64, t368: f64, t384: f64, t398: f64) -> (f64, f64, f64, f64, f64) {
    let t13133 = t1036 * t1089 * t175 * t864 * t839;
    let t13135 = t3244 * t1103;
    let t13137 = t1005 * t3493;
    let t13146 = t1005 * t3292;
    let t13156 = t384 * t398 * t368 * t3101 * t360;
    (t13133, t13135, t13137, t13146, t13156)
}
