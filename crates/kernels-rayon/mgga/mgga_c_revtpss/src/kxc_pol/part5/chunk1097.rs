//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1097/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1097(t1626: f64, t3011: f64, t15125: f64, t15191: f64, t4644: f64, t945: f64, t1614: f64, t2967: f64, t2986: f64, t4587: f64, t914: f64, t1596: f64, t2923: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15350 = t1626 * t3011;
    let t15363 = 0.2283111111111111111e-1_f64 * t15125;
    let t15364 = 0.11415555555555555555e-1_f64 * t15191;
    let t15400 = t4644 * t945;
    let t15406 = t1614 * t2967;
    let t15413 = t1626 * t2986;
    let t15416 = t4587 * t914;
    let t15421 = t1596 * t2923;
    (t15350, t15363, t15364, t15400, t15406, t15413, t15416, t15421)
}
