//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 853/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk853<F: Float>(t29917: F, t79: F, t781: F, t12198: F, t28368: F, t5006: F, t18701: F, t2013: F, t24967: F, t25128: F, t25131: F, t2630: F, t2634: F, t2638: F, t2644: F, t29891: F, t7581: F, t7591: F, t788: F, t9173: F, t9178: F, t9184: F, t9208: F, t9228: F) -> (F,) {
    let t29918 = t79 * t29917;
    let t29919 = t29918 * t781;
    let t29937 = t12198 * t28368;
    let t29938 = t5006 * t29937;
    let t29943 = 0.35981577432354634427e-1 * t7581 * t9173 + 0.27985671336275826777e-1 * t2013 * t29891 + 0.2698618307426597582e-1 * t29919 * t788 + 0.21588946459412780656e0 * t2634 * t9228 + 0.16191709844559585492e0 * t2630 * t9178 - 0.8095854922279792746e-1 * t2630 * t9228 - 0.7915947035118019574e0 * t9184 * t2644 - 0.8095854922279792746e-1 * t9208 * t2644 + 0.17990788716177317213e-1 * t18701 + 0.14392630972941853771e0 * t25128 + 0.53972366148531951639e-1 * t25131 - 0.95950873152945691806e-1 * t7591 * t9173 - 0.71963154864709268855e-1 * t2013 * t29938 + 0.26386490117060065246e0 * t24967 * t2638;
    (t29943,)
}
