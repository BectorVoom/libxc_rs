//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 705/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk705<F: Float>(t1821: F, t712: F, t1691: F, t188: F, t1906: F, t1647: F, t652: F, t390: F, t631: F, t644: F, t1659: F, t189: F) -> (F, F, F, F) {
    let t5593 = t712 * t1821;
    let t5594 = t5593 * t1691;
    let t5597 = t1906 * t188;
    let t5598 = t652 * t1647;
    let t5599 = t5597 * t5598;
    let t5601 = F::new(0.51550785283058921156e1) * t390 * t5599;
    let t5602 = t631 * t644;
    let t5603 = t5602 * t1659;
    let t5605 = F::new(0.21369999999999999999e0) * t390 * t5603;
    let t5606 = t189 * t1647;
    (t5594, t5601, t5605, t5606)
}
