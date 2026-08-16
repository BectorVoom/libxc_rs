//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 915/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk915(t8548: f64, t9763: f64, t3054: f64, t9080: f64, t1107: f64, t8549: f64, t3308: f64, t8229: f64, t1183: f64, t123: f64, t2349: f64, t8220: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9764 = t8548 * t9763;
    let t9765 = t9080 * t3054;
    let t9786 = t8549 * t1107;
    let t9787 = t8548 * t9786;
    let t9839 = 0.21687162600603479684e-1_f64 * t3308 * t8229;
    let t9840 = t1183 * t123;
    let t9841 = t9840 * t2349;
    let t9844 = 0.16265371950452609763e-1_f64 * t3308 * t8220;
    (t9764, t9765, t9787, t9839, t9841, t9844)
}
