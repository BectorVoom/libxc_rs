//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1001/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1001<F: Float>(t7637: F, t8571: F, t1998: F, t5251: F, t1967: F, t8566: F, t4557: F, t309: F, t556: F, t322: F, t406: F, t944: F, t1539: F, t463: F, t157: F, t1658: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t36386 = t7637 * t8571;
    let t36388 = t1998 * t5251;
    let t36390 = t1967 * t8566;
    let t36392 = t1998 * t4557;
    let t36416 = t556 * t309;
    let t36417 = t36416 * t322;
    let t36429 = t944 * t309 * t406;
    let t36475 = t1539 * t309;
    let t36479 = t1539 * t463;
    let t36495 = t157 * t463 * t309;
    let t36511 = t1658 * t406 * t157;
    (t36386, t36388, t36390, t36392, t36417, t36429, t36475, t36479, t36495, t36511)
}
