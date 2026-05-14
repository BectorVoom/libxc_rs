//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 738/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk738<F: Float>(t2021: F, t9226: F, t1586: F, t2013: F, t2630: F, t2634: F, t2638: F, t2644: F, t5479: F, t7578: F, t7581: F, t7589: F, t7591: F, t7603: F, t7625: F, t782: F, t788: F, t9169: F, t9173: F, t9178: F, t9184: F, t9189: F, t9208: F, t9214: F, t9218: F) -> (F, F, F) {
    let t9227 = t2021 * t9226;
    let t9228 = t1586 * t9227;
    let t9234 = 0.89953943580886586067e-2 * t2013 * t9169 + 0.11993859144118211476e-1 * t2013 * t9173 + 0.5397236614853195164e-1 * t782 * t9178 - 0.5397236614853195164e-1 * t2630 * t2644 + 0.26386490117060065246e0 * t9184 * t788 + 0.14392630972941853771e0 * t2634 * t2644 - 0.14392630972941853771e0 * t9189 * t788 + 0.2698618307426597582e-1 * t9208 * t788 + 0.59969295720591057378e-2 * t7603 - 0.17990788716177317213e-1 * t2013 * t9214 - 0.17990788716177317213e-1 * t2013 * t9218 + 0.17990788716177317213e-1 * t7581 * t2638 - 0.47975436576472845902e-1 * t7591 * t2638 - 0.2698618307426597582e-1 * t782 * t9228 - t5479 + 0.17990788716177317213e-1 * t7578 - 0.47975436576472845902e-1 * t7589 - 0.17990788716177317213e-1 * t7625;
    (t9227, t9228, t9234)
}
