//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 863/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk863(t10250: f64, t4518: f64, t2775: f64, t343: f64, t2244: f64, t2988: f64, t2987: f64, t3014: f64, t2990: f64, t2262: f64, t972: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10251 = t4518 * t10250;
    let t10254 = t343 * t2775;
    let t10255 = t10254 * t2244;
    let t10256 = t2988 * t10255;
    let t10259 = t2987 * t3014;
    let t10260 = t10259 * t2990;
    let t10263 = t2262 * t972;
    (t10251, t10254, t10255, t10256, t10259, t10260, t10263)
}
