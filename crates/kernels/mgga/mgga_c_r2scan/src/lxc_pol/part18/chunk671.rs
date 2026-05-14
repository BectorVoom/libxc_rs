//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 671/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk671<F: Float>(t1821: F, t712: F, t1691: F, t188: F, t1906: F, t1647: F, t652: F, t390: F, t631: F, t644: F, t1659: F, t189: F, t1663: F, t1923: F, t1981: F, t2006: F, t5435: F, t5439: F, t5454: F, t5474: F, t5479: F, t5564: F, t5567: F, t5569: F, t5572: F, t5585: F, t5589: F, t682: F, t687: F, t690: F) -> (F, F, F, F) {
    let t5593 = t712 * t1821;
    let t5594 = t5593 * t1691;
    let t5597 = t1906 * t188;
    let t5598 = t652 * t1647;
    let t5599 = t5597 * t5598;
    let t5601 = 0.51550785283058921156e1 * t390 * t5599;
    let t5602 = t631 * t644;
    let t5603 = t5602 * t1659;
    let t5605 = 0.21369999999999999999e0 * t390 * t5603;
    let t5606 = t189 * t1647;
    let t5607 = t1663 * t5606;
    let t5609 = 0.32055e0 * t390 * t5607;
    let t5610 = t5454 + t5474 - t5479 + 0.10526802520742363173e2 * t5564 * t5435 + 0.30762056574649219974e4 * t5567 * t5569 - 0.31168546390226634765e3 * t5572 * t5439 - t5585 + 18.0 * t687 * t682 * t1923 + 0.11579025239058625248e4 * t2006 * t690 * t5589 + 0.30762056574649219973e4 * t1981 * t5594 - t5601 - t5605 + t5609;
    (t5601, t5605, t5609, t5610)
}
