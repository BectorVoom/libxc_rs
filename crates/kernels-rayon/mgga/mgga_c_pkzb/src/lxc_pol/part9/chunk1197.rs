//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1197/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1197(t20788: f64, t672: f64, t665: f64, t1862: f64, t2759: f64, t5511: f64, t5547: f64, t1073: f64, t17432: f64, t5512: f64, t17444: f64, t1873: f64, t667: f64, t7360: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20789 = t672 * t20788;
    let t20791 = t665 * t20788;
    let t20794 = t5511 * t2759 * t1862;
    let t20797 = t5547 * t2759 * t1862;
    let t20800 = t17432 * t1073 * t5512;
    let t20803 = t17444 * t1073 * t5512;
    let t20806 = t1873 * t7360 * t667;
    (t20789, t20791, t20794, t20797, t20800, t20803, t20806)
}
