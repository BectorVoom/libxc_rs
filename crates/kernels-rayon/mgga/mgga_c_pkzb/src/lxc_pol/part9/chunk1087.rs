//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1087/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1087(t1947: f64, t1954: f64, t5498: f64, t709: f64, t1976: f64, t5490: f64, t1953: f64, t1975: f64, t252: f64, t5749: f64, t663: f64, t1847: f64, t1898: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17621 = t1947 * t1954;
    let t17624 = t709 * t5498;
    let t17630 = t1947 * t1976;
    let t17633 = t709 * t5490;
    let t17637 = 1.0_f64 / t1975 / t1953;
    let t17638 = t252 * t17637;
    let t17650 = t5749 * t663;
    let t17655 = t1847 * t1898;
    (t17621, t17624, t17630, t17633, t17637, t17638, t17650, t17655)
}
