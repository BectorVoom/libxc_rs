//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 836/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk836<F: Float>(t1782: F, t7351: F, t142: F, t2060: F, t1801: F, t2041: F, t1805: F, t1788: F, t7332: F, t1809: F, t570: F, t1797: F) -> (F, F, F, F, F, F, F, F) {
    let t9733 = t7351 * t1782;
    let t9734 = t142 * t9733;
    let t9735 = t2060 * t9734;
    let t9739 = t2041 * t1801;
    let t9741 = t2041 * t1805;
    let t9743 = t7332 * t1788;
    let t9747 = t570 * t1809;
    let t9749 = t570 * t1797;
    (t9733, t9734, t9735, t9739, t9741, t9743, t9747, t9749)
}
