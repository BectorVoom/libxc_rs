//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1191/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1191<F: Float>(t1839: F, t309: F, t157: F, t1937: F, t406: F, t1844: F, t463: F, t1658: F, t524: F, t1815: F, t301: F, t9476: F) -> (F, F, F, F, F, F, F, F) {
    let t40703 = t1839 * t309;
    let t40709 = t1937 * t406 * t157;
    let t40733 = t1844 * t309;
    let t40740 = t1844 * t463;
    let t40749 = t1658 * t524 * t157;
    let t40861 = t1815 * t309;
    let t40868 = t1815 * t463;
    let t40955 = t9476 * t301;
    (t40703, t40709, t40733, t40740, t40749, t40861, t40868, t40955)
}
