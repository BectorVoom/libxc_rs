//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 900/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk900<F: Float>(t30989: F, t7637: F, t7796: F, t1004: F, t390: F, t7613: F, t151: F, t37: F, t56: F, t593: F, t7508: F, t141: F, t420: F) -> (F, F, F, F, F) {
    let t30990 = F::cast_from(0.28582678745379824648e-2_f64) * t30989;
    let t30993 = t7637 * t7796;
    let t31001 = t1004 * t7613 * t390;
    let t31002 = F::cast_from(0.12004725073059526352e-1_f64) * t31001;
    let t31009 = t151 * t593 / t7508 / t37 * t56;
    let t31010 = t420 * t141;
    (t30990, t30993, t31002, t31009, t31010)
}
