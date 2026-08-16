//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2158/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2158<F: Float>(t13191: F, t25119: F, t841: F, t1878: F, t81982: F, t13184: F, t221: F, t25120: F, t6604: F, t81962: F, t13196: F, t13204: F, t6581: F) -> (F, F, F, F, F) {
    let t87418 = t25119 * t841 * t13191;
    let t87420 = t1878 * t81982;
    let t87422 = t87420 * t221 * t13184;
    let t87425 = t81962 * t6604 * t25120;
    let t87426 = F::cast_from(0.11869590291677274911e0_f64) * t87425;
    let t87428 = t25119 * t841 * t13196;
    let t87430 = t6581 * t13204;
    (t87418, t87422, t87426, t87428, t87430)
}
