//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 773/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk773<F: Float>(t1567: F, t910: F, t1570: F, t2124: F, t2562: F, t360: F, t2719: F, t481: F, t551: F, t552: F, t1632: F, t2634: F, t2184: F, t2612: F, t1592: F, t7195: F) -> (F, F, F, F, F, F, F) {
    let t7533 = t1567 * t910;
    let t7535 = t2124 * t7533 * t1570;
    let t7538 = t2562 * t1570;
    let t7539 = t360 * t7538;
    let t7542 = t2719 * t481;
    let t7544 = t551 * t552 * t7542;
    let t7551 = t551 * t1632 * t2634;
    let t7553 = 0.46230515946956099004e0 * t2184 * t7551;
    let t7555 = t551 * t1632 * t2612;
    let t7557 = 0.69345773920434148506e0 * t1592 * t7555;
    let t7561 = t551 * t552 * t7195;
    (t7535, t7538, t7539, t7544, t7553, t7557, t7561)
}
