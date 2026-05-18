//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 834/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk834<F: Float>(t1567: F, t910: F, t1570: F, t2124: F, t2562: F, t360: F, t2719: F, t481: F, t551: F, t552: F, t1632: F, t2634: F) -> (F, F, F, F, F) {
    let t7533 = t1567 * t910;
    let t7535 = t2124 * t7533 * t1570;
    let t7538 = t2562 * t1570;
    let t7539 = t360 * t7538;
    let t7542 = t2719 * t481;
    let t7544 = t551 * t552 * t7542;
    let t7551 = t551 * t1632 * t2634;
    (t7535, t7538, t7539, t7544, t7551)
}
