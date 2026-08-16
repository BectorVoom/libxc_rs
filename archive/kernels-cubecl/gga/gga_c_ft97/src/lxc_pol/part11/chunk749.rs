//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 749/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk749<F: Float>(t10121: F, t762: F, t242: F, t1882: F, t2591: F, t2596: F, t265: F, t724: F, t9596: F, t726: F, t8232: F, t2619: F, t684: F) -> (F, F, F, F, F, F, F) {
    let t10122 = t762 * t10121;
    let t10123 = t242 * t10122;
    let t10126 = t1882 * t2591;
    let t10128 = t1882 * t2596;
    let t10131 = t724 * t265 * t9596;
    let t10134 = t8232 * t726;
    let t10137 = t724 * t2619 * t684;
    (t10122, t10123, t10126, t10128, t10131, t10134, t10137)
}
