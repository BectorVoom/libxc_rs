//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 670/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk670<F: Float>(t2: F, t9895: F, t1775: F, t2499: F, t2494: F, t740: F, t8282: F, t9802: F, t2512: F, t458: F, t249: F, t3051: F) -> (F, F, F, F, F, F, F) {
    let t9896 = t9895 * t2;
    let t9903 = t1775 * t2499;
    let t9905 = t1775 * t2494;
    let t9907 = t8282 * t740;
    let t9916 = t9802 * t2;
    let t9933 = t458 * t2512;
    let t9935 = F::new(28.0) / F::new(27.0) * t3051 * t249;
    (t9896, t9903, t9905, t9907, t9916, t9933, t9935)
}
