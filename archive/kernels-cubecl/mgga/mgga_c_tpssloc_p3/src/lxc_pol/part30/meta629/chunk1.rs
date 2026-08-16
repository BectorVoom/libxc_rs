//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2035/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2035<F: Float>(t87411: F, t1878: F, t81982: F, t25120: F, t6604: F, t81962: F, t7500: F, t81911: F, t22690: F, t23122: F, t4119: F, t841: F) -> (F, F, F, F, F) {
    let t87412 = F::cast_from(0.28260929265898273598e-2_f64) * t87411;
    let t87420 = t1878 * t81982;
    let t87425 = t81962 * t6604 * t25120;
    let t87426 = F::cast_from(0.11869590291677274911e0_f64) * t87425;
    let t87432 = t81911 * t7500;
    let t87443 = t23122 * t22690 * t841 * t4119;
    (t87412, t87420, t87426, t87432, t87443)
}
