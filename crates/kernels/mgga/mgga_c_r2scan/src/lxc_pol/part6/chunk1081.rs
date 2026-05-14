//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1081/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1081<F: Float>(t19129: F, t297: F, t6649: F, t818: F, t1269: F, t1275: F, t6660: F, t815: F, t312: F, t320: F, t6659: F, t19091: F, t317: F, t325: F, t326: F, t6691: F) -> (F, F, F, F, F, F, F) {
    let t19131 = 1.0 / t297 / t19129;
    let t19138 = t6649 * t818;
    let t19141 = t1269 * t1275;
    let t19146 = t815 * t6660;
    let t19155 = t312 / t6659 / t320;
    let t19182 = 2618.0 / 81.0 * t317 * t19091;
    let t19203 = t325 / t6691 / t326;
    (t19131, t19138, t19141, t19146, t19155, t19182, t19203)
}
