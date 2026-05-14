//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 973/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk973<F: Float>(t12207: F, t9823: F, t41528: F, t41532: F, t41534: F, t41544: F, t44170: F, t44174: F, t44178: F, t44179: F, t44180: F, t44181: F, t44185: F, t44186: F, t13846: F, t1841: F, t2536: F, t734: F) -> (F, F) {
    let t47572 = t9823 * t12207;
    let t47574 = 0.38342925953920749677e0 * t41528;
    let t47575 = 0.85206502119823888171e-1 * t41532;
    let t47576 = 0.38342925953920749677e0 * t41534;
    let t47578 = -t44170 - t44174 - t44178 + 0.35750489951850426669e0 * t47572 - t44179 - t44180 + t44181 - t47574 + t47575 - t47576 + t44185 + t44186 - 0.76685851907841499354e0 * t41544;
    let t47587 = t1841 * t2536 * t13846 * t734;
    (t47578, t47587)
}
