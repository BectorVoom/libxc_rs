//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1188/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1188(t11713: f64, t1210: f64, t53081: f64, t11647: f64, t1731: f64, t11718: f64, t52835: f64, t1744: f64, t11716: f64, t1174: f64, t1725: f64, t2402: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53087 = t11713 * t1210 * t53081;
    let t53099 = t1731 * t11647;
    let t53238 = t52835 * t11718;
    let t53274 = t1744 * t11647;
    let t53336 = t11713 * t11716 * t53081;
    let t53440 = t1174 * t2402 * t1725;
    (t53087, t53099, t53238, t53274, t53336, t53440)
}
