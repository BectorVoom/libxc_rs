//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 585/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk585<F: Float>(t23898: F, t358: F, t5842: F, t1359: F, t1557: F, t1882: F, t5918: F, t375: F, t5925: F, t89: F, t1374: F, t1636: F) -> (F, F, F, F, F, F) {
    let t23899 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t23898;
    let t23900 = t5842 * t358;
    let t23909 = t1359 * t1557;
    let t23914 = t1882 * t5918;
    let t23920 = t89 * t375 * t5925;
    let t23923 = t89 * t1636 * t1374;
    (t23899, t23900, t23909, t23914, t23920, t23923)
}
