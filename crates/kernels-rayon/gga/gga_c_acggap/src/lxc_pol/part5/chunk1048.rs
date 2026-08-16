//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1048/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1048(t224: f64, t4068: f64, t1390: f64, t709: f64, t12930: f64, t1549: f64, t1554: f64, t1558: f64, t13263: f64, t1545: f64, t3379: f64, t4291: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18217 = t224 * t4068;
    let t18222 = t709 * t1390;
    let t18295 = t12930 * t1549;
    let t18297 = t12930 * t1554;
    let t18299 = t12930 * t1558;
    let t18301 = t13263 * t1545;
    let t18303 = t3379 * t4291;
    (t18217, t18222, t18295, t18297, t18299, t18301, t18303)
}
