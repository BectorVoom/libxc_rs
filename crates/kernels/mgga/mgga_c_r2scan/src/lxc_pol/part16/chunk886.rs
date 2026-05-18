//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 886/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk886<F: Float>(t2148: F, t9445: F, t2147: F, t1632: F, t3216: F, t551: F, t549: F, t2670: F, t2731: F, t133: F, t255: F, t3177: F) -> (F, F, F, F, F) {
    let t9446 = t2148 * t9445;
    let t9447 = t2147 * t9446;
    let t9451 = t1632 * t3216;
    let t9452 = t551 * t9451;
    let t9453 = t549 * t9452;
    let t9458 = t2670 * t2731;
    let t9463 = t133 * t3177 * t255;
    (t9447, t9451, t9453, t9458, t9463)
}
