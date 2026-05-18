//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 782/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk782<F: Float>(t1775: F, t2778: F, t2767: F, t303: F, t3051: F, t1771: F, t854: F, t848: F, t9909: F, t4206: F, t9592: F, t10491: F, t2: F) -> (F, F, F, F, F, F, F) {
    let t10589 = t1775 * t2778;
    let t10591 = t1775 * t2767;
    let t10594 = F::new(28.0) / F::new(27.0) * t3051 * t303;
    let t10595 = t1771 * t854;
    let t10597 = t848 * t9909;
    let t10600 = t4206 * t9592;
    let t10603 = t10491 * t2;
    (t10589, t10591, t10594, t10595, t10597, t10600, t10603)
}
