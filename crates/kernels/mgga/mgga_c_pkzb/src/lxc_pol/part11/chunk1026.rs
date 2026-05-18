//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1026/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1026<F: Float>(t11345: F, t405: F, t6462: F, t758: F, t11159: F, t11238: F, t11240: F, t11242: F, t11245: F, t11263: F, t11266: F, t11316: F, t11321: F, t11325: F, t11329: F) -> (F, F, F, F) {
    let t11346 = t405 * t11345;
    let t11347 = t11346 * t6462;
    let t11348 = t758 * t11347;
    let t11351 = -t11159 + t11238 + t11240 + t11242 - t11245 + t11263 + t11266 + t11316 - t11321 - t11325 + t11329;
    (t11346, t11347, t11348, t11351)
}
