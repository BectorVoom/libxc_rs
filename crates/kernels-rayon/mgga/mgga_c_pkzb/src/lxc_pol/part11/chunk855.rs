//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 855/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk855(t1066: f64, t2739: f64, t218: f64, t219: f64, t3546: f64, t675: f64) -> (f64, f64, f64) {
    let t9187 = t1066 * t2739;
    let t9189 = t218 * t219 * t9187;
    let t9192 = t218 * t675 * t3546;
    (t9187, t9189, t9192)
}
