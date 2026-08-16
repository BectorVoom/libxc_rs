//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 717/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk717(t1663: f64, t5606: f64, t390: f64, t1923: f64, t1981: f64, t2006: f64, t5435: f64, t5439: f64, t5454: f64, t5474: f64, t5479: f64, t5564: f64, t5567: f64, t5569: f64, t5572: f64, t5585: f64, t5589: f64, t5594: f64, t5601: f64, t5605: f64, t682: f64, t687: f64, t690: f64) -> (f64, f64) {
    let t5607 = t1663 * t5606;
    let t5609 = 0.32055e0_f64 * t390 * t5607;
    let t5610 = t5454 + t5474 - t5479 + 0.10526802520742363173e2_f64 * t5564 * t5435 + 0.30762056574649219974e4_f64 * t5567 * t5569 - 0.31168546390226634765e3_f64 * t5572 * t5439 - t5585 + 18.0_f64 * t687 * t682 * t1923 + 0.11579025239058625248e4_f64 * t2006 * t690 * t5589 + 0.30762056574649219973e4_f64 * t1981 * t5594 - t5601 - t5605 + t5609;
    (t5609, t5610)
}
