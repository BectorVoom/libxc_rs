//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2757/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2757(t4119: f64, t868: f64, t12652: f64, t12939: f64, t4195: f64, t1462: f64, t47172: f64) -> (f64, f64, f64, f64) {
    let t58071 = t4119 * t868;
    let t58080 = 96.0_f64 * t12939 * t4195 * t12652;
    let t58085 = 8.0_f64 * t47172 * t1462;
    let t58090 = t4119 * t4119;
    (t58071, t58080, t58085, t58090)
}
