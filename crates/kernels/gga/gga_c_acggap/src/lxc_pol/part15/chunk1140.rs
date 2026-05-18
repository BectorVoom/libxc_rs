//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1140/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1140<F: Float>(t2060: F, t372: F, t8927: F, t9563: F, t5694: F, t8806: F, t5698: F, t7436: F, t1839: F, t322: F, t1181: F, t599: F, t7346: F) -> (F, F, F, F, F) {
    let t39733 = t2060 * t8927 * t9563 * t372;
    let t39735 = t8806 * t5694;
    let t39737 = t7436 * t5698;
    let t39743 = t1839 * t322;
    let t39746 = t7346 * t1181 * t599 * t39743;
    (t39733, t39735, t39737, t39743, t39746)
}
