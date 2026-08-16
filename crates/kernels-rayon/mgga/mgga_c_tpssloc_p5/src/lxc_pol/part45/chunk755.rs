//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 755/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk755(t23092: f64, t23149: f64, t235: f64, t234: f64, t852: f64, t776: f64, t6637: f64, t6552: f64, t2553: f64, t6638: f64, t117: f64, t229: f64, t67: f64) -> (f64, f64, f64, f64, f64) {
    let t23150 = t23092 + t23149;
    let t23151 = t235 * t23150;
    let t23153 = t234 * t852;
    let t23154 = t23153 * t776;
    let t23155 = t6637 * t23154;
    let t23156 = t6552 * t23155;
    let t23158 = t6638 * t2553;
    let t23159 = t6637 * t23158;
    let t23160 = t6552 * t23159;
    let t23163 = t229 * t67 * t117;
    (t23150, t23151, t23156, t23160, t23163)
}
