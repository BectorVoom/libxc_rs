//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 511/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk511<F: Float>(t2719: F, t552: F, t551: F, t1632: F, t938: F, t549: F, t910: F, t566: F, t378: F, t5: F, t966: F, t750: F, t963: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2720 = t552 * t2719;
    let t2721 = t551 * t2720;
    let t2726 = t1632 * t938;
    let t2727 = t551 * t2726;
    let t2728 = t549 * t2727;
    let t2730 = t1632 * t910;
    let t2731 = t551 * t2730;
    let t2732 = t566 * t2731;
    let t2736 = t5 * t378 * t966;
    let t2738 = t963 * t750;
    (t2720, t2721, t2726, t2727, t2728, t2730, t2731, t2732, t2736, t2738)
}
