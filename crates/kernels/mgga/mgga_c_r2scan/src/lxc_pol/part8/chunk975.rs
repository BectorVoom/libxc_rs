//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 975/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk975<F: Float>(t2719: F, t938: F, t551: F, t552: F, t2207: F, t2691: F, t2837: F, t3016: F, t788: F, t785: F, t113: F, t8837: F, t2115: F, t1604: F, t529: F, t538: F, t8692: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9407 = t938 * t2719;
    let t9409 = t551 * t552 * t9407;
    let t9416 = t2207 * t2837 * t2691;
    let t9418 = t788 * t3016;
    let t9420 = t2207 * t785 * t9418;
    let t9422 = t8837 * t113;
    let t9423 = t2115 * t9422;
    let t9424 = t1604 * t9423;
    let t9427 = t529 * t538 * t8692;
    (t9407, t9409, t9416, t9418, t9420, t9422, t9423, t9424, t9427)
}
