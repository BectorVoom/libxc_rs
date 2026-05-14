//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1119/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1119<F: Float>(t1111: F, t4554: F, t1128: F, t1114: F, t4550: F, t1112: F, t1123: F, t1127: F, t1132: F, t1158: F, t11931: F, t11936: F, t11960: F, t11965: F, t11968: F, t12018: F, t12021: F, t12024: F, t12050: F, t12054: F, t2900: F, t2915: F, t2946: F, t2953: F, t2957: F, t3729: F, t3771: F, t4610: F, t4636: F, t4639: F, t7897: F, t9868: F, t9873: F) -> (F, F, F, F) {
    let t12058 = t4554 * t1111;
    let t12059 = t1128 * t12058;
    let t12062 = t4554 * t1114;
    let t12063 = t1128 * t12062;
    let t12072 = t4550 * t1111;
    let t12101 = -0.12e-1 * t2900 * t12059 + 0.18e-1 * t2946 * t12063 + 0.16e-1 * t1132 * t12018 - 0.256e-3 * t1127 * t12021 + 0.256e-3 * t1132 * t12024 + 0.9e-1 * t2953 * t1128 * t12072 - 0.108e0 * t2915 * t12050 - 0.48e-1 * t1158 * t12054 - 0.108e0 * t2915 * t12059 + 0.126e0 * t2957 * t12063 - 0.32e-1 * t3729 * t4610 - t1112 * t4639 - 4.0 * t4636 * t1123 - 0.1536e-2 * t9868 * t11960 + 0.53333333333333333333e0 * t9868 * t11965 + 0.1536e-2 * t9873 * t11968 + 0.53333333333333333333e0 * t9873 * t11965 + 0.9216e-8 * t3771 * t11931 - 0.192e-3 * t7897 * t11936;
    (t12058, t12062, t12072, t12101)
}
