//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2714/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2714(t17217: f64, t17505: f64, t1032: f64, t1246: f64, t21333: f64, t17720: f64, t5391: f64, t11262: f64, t3610: f64, t6634: f64, t17569: f64, t5326: f64, t5390: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t69947 = t17505 * t17217;
    let t69958 = t21333 * t1032 * t1246;
    let t69961 = t5391 * t17720;
    let t69964 = t3610 * t11262 * t6634;
    let t69966 = t17569 * t17217;
    let t69968 = t5326 * t5390;
    (t69947, t69958, t69961, t69964, t69966, t69968)
}
