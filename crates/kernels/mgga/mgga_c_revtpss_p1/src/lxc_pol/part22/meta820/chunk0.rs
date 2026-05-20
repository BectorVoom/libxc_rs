//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2933/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2933<F: Float>(t4132: F, t5599: F, t689: F, t14103: F, t9285: F, t9674: F, t13730: F, t1420: F, t2782: F, t13726: F, t9303: F, t13725: F, t1445: F, t2439: F) -> (F, F, F, F, F) {
    let t47929 = t689 * t5599 * t4132;
    let t47932 = t9674 * t14103 * t9285;
    let t47936 = t2782 * t1420 * t13730;
    let t47938 = t9303 * t13726;
    let t47942 = t2439 * t13725 * t1445;
    (t47929, t47932, t47936, t47938, t47942)
}
