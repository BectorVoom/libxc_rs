//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 659/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk659<F: Float>(t157: F, t9132: F, t2101: F, t605: F, t9071: F, t151: F, t3051: F, t1771: F, t588: F, t2: F, t9114: F, t583: F, t8282: F) -> (F, F, F, F, F, F, F) {
    let t9133 = t9132 * t157;
    let t9144 = t2101 * t605;
    let t9166 = F::new(28.0) / F::new(27.0) * t9071;
    let t9178 = F::new(28.0) / F::new(27.0) * t3051 * t151;
    let t9179 = t1771 * t588;
    let t9192 = t9114 * t2;
    let t9202 = t8282 * t583;
    (t9133, t9144, t9166, t9178, t9179, t9192, t9202)
}
