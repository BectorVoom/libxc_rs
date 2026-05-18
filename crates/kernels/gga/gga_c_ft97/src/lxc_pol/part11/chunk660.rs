//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 660/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk660<F: Float>(t9071: F, t2086: F, t2120: F, t590: F, t91: F, t151: F, t3051: F, t1771: F, t588: F, t2102: F, t9041: F, t9045: F) -> (F, F, F, F, F, F) {
    let t9166 = F::new(28.0) / F::new(27.0) * t9071;
    let t9170 = t91 * t2086 * t590 * t2120;
    let t9178 = F::new(28.0) / F::new(27.0) * t3051 * t151;
    let t9179 = t1771 * t588;
    let t9181 = t2102 * t9041;
    let t9183 = t2102 * t9045;
    (t9166, t9170, t9178, t9179, t9181, t9183)
}
