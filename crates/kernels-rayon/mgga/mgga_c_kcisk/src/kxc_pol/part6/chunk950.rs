//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 950/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk950(t2063: f64, t5491: f64, t9226: f64, t1775: f64, t18356: f64, t18406: f64, t18408: f64, t2013: f64, t24876: f64, t24880: f64, t24908: f64, t24910: f64, t24913: f64, t24921: f64, t24926: f64, t2638: f64, t7581: f64, t7591: f64, t9214: f64, t9218: f64) -> f64 {
    let t29789 = t5491 * t2063 * t9226;
    let t29790 = t1775 * t29789;
    let t29807 = 0.14392630972941853771e0_f64 * t7591 * t9214 - 0.2698618307426597582e-1_f64 * t2013 * t29790 - 0.47975436576472845903e-1_f64 * t24908 + 0.17990788716177317214e-1_f64 * t24910 - 0.17990788716177317214e-1_f64 * t24913 + 0.89953943580886586067e-2_f64 * t24921 + 0.11993859144118211476e-1_f64 * t24926 - 0.14392630972941853771e0_f64 * t24876 * t2638 + 0.2698618307426597582e-1_f64 * t24880 * t2638 - 0.53972366148531951639e-1_f64 * t7581 * t9218 - 0.59969295720591057378e-2_f64 * t18356 - 0.17990788716177317213e-1_f64 * t18406 + 0.47975436576472845902e-1_f64 * t18408;
    t29807
}
