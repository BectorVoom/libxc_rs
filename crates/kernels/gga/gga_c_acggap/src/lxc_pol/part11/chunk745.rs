//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 745/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk745<F: Float>(t355: F, t535: F, t2095: F, t5720: F, t599: F, t1181: F, t7337: F, t5606: F, t604: F, t2068: F, t1165: F, t7351: F, t524: F, t944: F, t406: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8771 = t535 * t355;
    let t8772 = t2095 * t8771;
    let t8774 = t599 * t5720;
    let t8775 = t1181 * t8774;
    let t8776 = t7337 * t8775;
    let t8778 = t604 * t5606;
    let t8779 = t1181 * t8778;
    let t8780 = t2068 * t8779;
    let t8783 = t1165 * t604 * t5720;
    let t8784 = t7337 * t8783;
    let t8787 = t1165 * t7351 * t5606;
    let t8788 = t2068 * t8787;
    let t8790 = t524 * t944;
    let t8791 = t8790 * t406;
    (t8771, t8772, t8774, t8775, t8776, t8778, t8779, t8780, t8783, t8784, t8787, t8788, t8790, t8791)
}
