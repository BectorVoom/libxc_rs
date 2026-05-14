//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 622/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk622<F: Float>(t151: F, t3051: F, t1771: F, t588: F, t2102: F, t9041: F, t9045: F, t24: F, t586: F, t9007: F, t1775: F, t2103: F, t2106: F, t2: F, t9114: F, t9050: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9178 = 28.0 / 27.0 * t3051 * t151;
    let t9179 = t1771 * t588;
    let t9181 = t2102 * t9041;
    let t9183 = t2102 * t9045;
    let t9186 = t24 * t586 * t9007;
    let t9188 = t1775 * t2103;
    let t9190 = t1775 * t2106;
    let t9192 = t9114 * t2;
    let t9193 = t9192 * t9050;
    (t9178, t9179, t9181, t9183, t9186, t9188, t9190, t9192, t9193)
}
