//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2528/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2528<F: Float>(t1597: F, t341: F, t10245: F, t13847: F, t2986: F, t13931: F, t2987: F, t135: F, t13933: F, t973: F, t13532: F, t13784: F) -> (F, F, F, F, F) {
    let t48184 = t341 * t1597;
    let t48189 = t2986 * t13847 * t10245;
    let t48191 = t2987 * t13931;
    let t48207 = t973 * t135 * t13933;
    let t48210 = t2986 * t13784 * t13532;
    (t48184, t48189, t48191, t48207, t48210)
}
