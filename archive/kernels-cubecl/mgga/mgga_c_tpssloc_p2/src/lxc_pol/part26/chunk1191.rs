//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1191/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1191<F: Float>(t3242: F, t491: F, t3247: F, t24658: F, t3: F, t24719: F, t3030: F, t1215: F, t24815: F, t1011: F, t475: F, t497: F) -> (F, F, F, F, F, F, F, F) {
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
