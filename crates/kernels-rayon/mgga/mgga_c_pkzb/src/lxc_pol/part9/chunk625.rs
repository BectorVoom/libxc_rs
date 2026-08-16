//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 625/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk625(t2759: f64, t665: f64, t1073: f64, t1873: f64, t667: f64, t672: f64, t1079: f64, t218: f64, t675: f64, t1066: f64, t655: f64, t219: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2760 = t665 * t2759;
    let t2765 = t1873 * t1073;
    let t2766 = t2765 * t667;
    let t2768 = t672 * t2759;
    let t2772 = t218 * t675 * t1079;
    let t2774 = t655 * t1066;
    let t2776 = t218 * t219 * t2774;
    (t2760, t2765, t2766, t2768, t2772, t2774, t2776)
}
