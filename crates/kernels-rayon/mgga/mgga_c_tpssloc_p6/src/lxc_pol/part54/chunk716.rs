//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 716/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk716(t6914: f64, t1878: f64, t547: f64, t1329: f64, t1995: f64, t2230: f64, t213: f64, t1999: f64, t533: f64, t556: f64, t598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6915 = 7.0_f64 / 288.0_f64 * t6914;
    let t6916 = t1878 * t547;
    let t6917 = t6916 * t1329;
    let t6919 = t2230 * t1995;
    let t6920 = t6919 * t213;
    let t6921 = t6920 * t1999;
    let t6922 = 0.14130464632949136799e-2_f64 * t6921;
    let t6924 = 1.0_f64 / t556 / t533;
    let t6925 = t598 * t6924;
    let t6926 = t6925 * t213;
    (t6915, t6916, t6917, t6919, t6920, t6921, t6922, t6924, t6925, t6926)
}
