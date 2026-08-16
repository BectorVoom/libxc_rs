//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 662/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk662(t2013: f64, t2630: f64, t2634: f64, t2638: f64, t2644: f64, t5479: f64, t7578: f64, t7581: f64, t7589: f64, t7591: f64, t7603: f64, t7625: f64, t782: f64, t788: f64, t9169: f64, t9173: f64, t9178: f64, t9184: f64, t9189: f64, t9208: f64, t9214: f64, t9218: f64, t9228: f64) -> f64 {
    let t9234 = 0.89953943580886586067e-2_f64 * t2013 * t9169 + 0.11993859144118211476e-1_f64 * t2013 * t9173 + 0.5397236614853195164e-1_f64 * t782 * t9178 - 0.5397236614853195164e-1_f64 * t2630 * t2644 + 0.26386490117060065246e0_f64 * t9184 * t788 + 0.14392630972941853771e0_f64 * t2634 * t2644 - 0.14392630972941853771e0_f64 * t9189 * t788 + 0.2698618307426597582e-1_f64 * t9208 * t788 + 0.59969295720591057378e-2_f64 * t7603 - 0.17990788716177317213e-1_f64 * t2013 * t9214 - 0.17990788716177317213e-1_f64 * t2013 * t9218 + 0.17990788716177317213e-1_f64 * t7581 * t2638 - 0.47975436576472845902e-1_f64 * t7591 * t2638 - 0.2698618307426597582e-1_f64 * t782 * t9228 - t5479 + 0.17990788716177317213e-1_f64 * t7578 - 0.47975436576472845902e-1_f64 * t7589 - 0.17990788716177317213e-1_f64 * t7625;
    t9234
}
