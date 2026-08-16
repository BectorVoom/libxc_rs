//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 956/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk956(t29917: f64, t79: f64, t781: f64, t12198: f64, t28368: f64, t5006: f64, t18701: f64, t2013: f64, t24967: f64, t25128: f64, t25131: f64, t2630: f64, t2634: f64, t2638: f64, t2644: f64, t29891: f64, t7581: f64, t7591: f64, t788: f64, t9173: f64, t9178: f64, t9184: f64, t9208: f64, t9228: f64) -> f64 {
    let t29918 = t79 * t29917;
    let t29919 = t29918 * t781;
    let t29937 = t12198 * t28368;
    let t29938 = t5006 * t29937;
    let t29943 = 0.35981577432354634427e-1_f64 * t7581 * t9173 + 0.27985671336275826777e-1_f64 * t2013 * t29891 + 0.2698618307426597582e-1_f64 * t29919 * t788 + 0.21588946459412780656e0_f64 * t2634 * t9228 + 0.16191709844559585492e0_f64 * t2630 * t9178 - 0.8095854922279792746e-1_f64 * t2630 * t9228 - 0.7915947035118019574e0_f64 * t9184 * t2644 - 0.8095854922279792746e-1_f64 * t9208 * t2644 + 0.17990788716177317213e-1_f64 * t18701 + 0.14392630972941853771e0_f64 * t25128 + 0.53972366148531951639e-1_f64 * t25131 - 0.95950873152945691806e-1_f64 * t7591 * t9173 - 0.71963154864709268855e-1_f64 * t2013 * t29938 + 0.26386490117060065246e0_f64 * t24967 * t2638;
    t29943
}
