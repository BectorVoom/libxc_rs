//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 706/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk706<F: Float>(t292: F, t10292: F, t10297: F, t10359: F, t10364: F, t10365: F, t10369: F, t10384: F, t2688: F, t2691: F, t2692: F, t2720: F, t2726: F, t2735: F, t285: F, t4061: F, t4113: F, t800: F, t817: F, t821: F) -> (F,) {
    let t293 = 0.1e-59 < t292;
    let t10388 = piecewise3(t293, 12.0 * t10292 * t2691 * t2726 - 6.0 * t10364 * t10365 * t285 + 6.0 * t10369 * t2735 * t4113 - t10384 * t285 * t817 - 6.0 * t2691 * t2692 * t2735 - 6.0 * t10297 * t2691 + 2.0 * t10359 * t800 - 6.0 * t2688 * t821 + 6.0 * t2720 * t4061, 0.0);
    (t10388,)
}
