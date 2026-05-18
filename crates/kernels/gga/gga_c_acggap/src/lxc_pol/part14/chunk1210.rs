//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1210/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1210<F: Float>(t315: F, t40619: F, t2134: F, t1839: F, t309: F, t7932: F, t7963: F, t157: F, t1937: F, t406: F, t2132: F, t2138: F, t322: F, t9767: F) -> (F, F, F, F) {
    let t40697 = t315 * t40619;
    let t40698 = t40697 * t2134;
    let t40703 = t1839 * t309;
    let t40705 = t7963 * t7932 * t40703;
    let t40709 = t1937 * t406 * t157;
    let t40721 = t2138 * t2132 * t9767 * t322;
    (t40698, t40705, t40709, t40721)
}
