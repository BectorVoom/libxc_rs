//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 859/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk859(t2747: f64, t745: f64, t7007: f64, t76: f64, t5896: f64, t2810: f64, t595: f64, t637: f64, t2813: f64, t236: f64, t5709: f64, t5727: f64, t5736: f64, t5739: f64, t5889: f64, t5891: f64, t5895: f64) -> f64 {
    let t7802 = 0.11696447245269292414e1_f64 * t2747 * t745;
    let t7803 = t7007 * t76;
    let t7807 = 32.0_f64 * t5896;
    let t7808 = t595 * t2810;
    let t7810 = 0.40020429009866666666e-2_f64 * t7808 * t637;
    let t7811 = t595 * t2813;
    let t7813 = 0.40020429009866666666e-2_f64 * t7811 * t637;
    let t7814 = t7802 + 0.5848223622634646207e0_f64 * t7803 * t236 - t5889 - t5709 + t5727 - t5736 - t5739 - 2.0_f64 * t5891 - t5895 + t7807 - t7810 - t7813;
    t7814
}
