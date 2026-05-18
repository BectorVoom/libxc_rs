//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1240/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1240<F: Float>(t12392: F, t37282: F, t3579: F, t39312: F, t11515: F, t12422: F, t3618: F, t910: F, t3270: F, t10667: F, t11479: F, t11625: F, t3275: F) -> (F, F, F, F, F) {
    let t43787 = F::new(3.0) / F::new(2.0) * t37282 * t12392;
    let t43789 = t3579 * t39312 / F::new(2.0);
    let t43791 = t12422 * t11515 / F::new(4.0);
    let t43792 = t3618 * t910;
    let t43793 = t3270 * t43792;
    let t43795 = F::new(3.0) / F::new(2.0) * t10667 * t43793;
    let t43797 = t3275 * t11479 * t11625;
    (t43787, t43789, t43791, t43795, t43797)
}
