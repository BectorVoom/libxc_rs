//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 653/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk653<F: Float>(t1882: F, t2218: F, t1554: F, t525: F, t157: F, t1984: F, t355: F, t2101: F, t605: F, t9071: F, t151: F, t3051: F) -> (F, F, F, F, F, F, F, F) {
    let t9112 = t1882 * t2218;
    let t9114 = t1554 * t525;
    let t9115 = t9114 * t157;
    let t9132 = t355 * t1984;
    let t9133 = t9132 * t157;
    let t9144 = t2101 * t605;
    let t9166 = F::new(28.0) / F::new(27.0) * t9071;
    let t9178 = F::new(28.0) / F::new(27.0) * t3051 * t151;
    (t9112, t9114, t9115, t9132, t9133, t9144, t9166, t9178)
}
