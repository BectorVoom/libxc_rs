//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1369/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1369<F: Float>(t11064: F, t42332: F, t11058: F, t3185: F, t42741: F, t10481: F, t1049: F, t3040: F, t3166: F, t1014: F, t42340: F, t42341: F) -> (F, F, F, F, F, F) {
    let t43470 = t42332 * t11064;
    let t43473 = t42332 * t11058;
    let t43480 = t42741 * t3185;
    let t43483 = t1049 * t10481;
    let t43489 = t3166 * t3040;
    let t43503 = t42340 * t42341 * t1014;
    (t43470, t43473, t43480, t43483, t43489, t43503)
}
