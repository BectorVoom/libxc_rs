//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 982/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk982<F: Float>(t12207: F, t9823: F, t41528: F, t41532: F, t41534: F, t13846: F, t1841: F, t2536: F, t734: F, t2558: F, t39002: F, t9647: F) -> (F, F, F, F, F, F) {
    let t47572 = t9823 * t12207;
    let t47574 = F::cast_from(0.38342925953920749677e0_f64) * t41528;
    let t47575 = F::cast_from(0.85206502119823888171e-1_f64) * t41532;
    let t47576 = F::cast_from(0.38342925953920749677e0_f64) * t41534;
    let t47587 = t1841 * t2536 * t13846 * t734;
    let t47594 = t9647 * t39002 * t2558;
    (t47572, t47574, t47575, t47576, t47587, t47594)
}
