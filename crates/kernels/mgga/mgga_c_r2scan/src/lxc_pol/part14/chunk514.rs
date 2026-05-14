//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 514/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk514<F: Float>(t44: F, t51: F, t109: F, t95: F, t1541: F, t910: F, t481: F, t1212: F, t889: F, t35: F, t472: F, t1216: F, t415: F, t1224: F, t893: F, t476: F, t419: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t2504 = t109 * t95;
    let t2505 = t1541 * t910;
    let t2506 = t2505 * t481;
    let t2509 = t1212 * t889;
    let t2512 = t472 * t35;
    let t2516 = piecewise3(t45, 0.0, -2.0 / 9.0 * t2509 * t415 + 4.0 / 3.0 * t2512 * t1216);
    let t2517 = t1224 * t893;
    let t2520 = t476 * t35;
    let t2524 = piecewise3(t52, 0.0, -2.0 / 9.0 * t2517 * t419 - 4.0 / 3.0 * t2520 * t1216);
    let t2526 = t2516 / 2.0 + t2524 / 2.0;
    (t2504, t2505, t2506, t2509, t2512, t2517, t2520, t2526)
}
