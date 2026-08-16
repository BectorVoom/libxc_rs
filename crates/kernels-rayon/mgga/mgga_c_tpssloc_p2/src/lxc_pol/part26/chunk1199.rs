//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1199/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1199(t1215: f64, t2250: f64, t2244: f64, t1388: f64, t3734: f64, t1351: f64, t3719: f64, t1307: f64, t3791: f64, t12240: f64, t1352: f64, t3850: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t52531 = t2250 * t1215;
    let t52537 = t2244 * t1215;
    let t53789 = t1388 * t3734;
    let t54542 = t1351 * t3734;
    let t54591 = t3719 * t1351;
    let t54770 = t3791 * t1307;
    let t54858 = t12240 * t1351;
    let t55003 = t1352 * t3850;
    (t52531, t52537, t53789, t54542, t54591, t54770, t54858, t55003)
}
