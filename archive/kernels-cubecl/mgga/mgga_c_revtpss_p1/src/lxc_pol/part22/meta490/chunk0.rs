//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2213/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2213<F: Float>(t16152: F, t247: F, t3116: F, t3109: F, t4583: F, t1063: F, t3172: F, t4868: F, t1041: F, t2862: F, t4823: F, t1042: F) -> (F, F, F, F, F, F, F) {
    let t16154 = t247 * t3116 * t16152;
    let t16158 = t247 * t3109 * t4583;
    let t16160 = F::cast_from(0.19055119163586549765e-3_f64) * t1063 * t16158;
    let t16163 = t3172 * t4868;
    let t16165 = F::cast_from(0.28582678745379824648e-3_f64) * t1041 * t16163;
    let t16166 = t4823 * t2862;
    let t16167 = t1042 * t16166;
    (t16154, t16158, t16160, t16163, t16165, t16166, t16167)
}
