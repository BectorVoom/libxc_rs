//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1191/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1191(t3242: f64, t491: f64, t3247: f64, t24658: f64, t3: f64, t24719: f64, t3030: f64, t1215: f64, t24815: f64, t1011: f64, t475: f64, t497: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27551 = t491 * t3242;
    let t27561 = t491 * t3247;
    let t27634 = t24658 * t3;
    let t27635 = t24719 * t3030;
    let t27636 = t27634 * t27635;
    let t27638 = t24815 * t1215;
    let t27643 = t1011 * t1215;
    let t27644 = t27643 * t475;
    let t27774 = t497 * t3242;
    (t27551, t27561, t27634, t27635, t27636, t27638, t27644, t27774)
}
