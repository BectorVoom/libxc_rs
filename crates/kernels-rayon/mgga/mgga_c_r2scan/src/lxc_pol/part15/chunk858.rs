//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 858/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk858(t1678: f64, t955: f64, t159: f64, t1686: f64, t170: f64, t7028: f64, t5474: f64, t5479: f64, t5585: f64, t5601: f64, t5605: f64, t5846: f64, t5847: f64, t5853: f64, t5855: f64, t5864: f64) -> f64 {
    let t7783 = t955 * t1678;
    let t7784 = t159 * t7783;
    let t7785 = t7784 * t1686;
    let t7788 = t7028 * t170;
    let t7792 = 0.42340699333333333333e-3_f64 * t7785 + t5474 - t5479 - t5846 + 24.0_f64 * t5847 + t5853 - t5585 + 0.285764e-1_f64 * t159 * t7788 - 0.1143056e0_f64 * t5855 - t5864 - t5601 - t5605;
    t7792
}
