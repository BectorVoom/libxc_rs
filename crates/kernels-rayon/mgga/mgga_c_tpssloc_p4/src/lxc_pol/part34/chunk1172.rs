//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1172/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1172(t1888: f64, t23270: f64, t25044: f64, t5657: f64, t1880: f64, t25224: f64, t28263: f64, t28276: f64, t6552: f64, t1484: f64, t25038: f64, t98169: f64) -> (f64, f64, f64, f64) {
    let t105437 = t1888 * t23270 * t25044 * t5657;
    let t105441 = t1880 * t25224 * t28263;
    let t105445 = t6552 * t25224 * t28276;
    let t105449 = t25038 * t23270 * t98169 * t1484;
    (t105437, t105441, t105445, t105449)
}
