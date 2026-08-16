//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 952/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk952(t1775: f64, t29832: f64, t12234: f64, t29819: f64, t12248: f64, t2013: f64, t24976: f64, t24978: f64, t24980: f64, t25007: f64, t25024: f64, t25027: f64, t29812: f64, t29816: f64, t29821: f64, t29826: f64, t7581: f64, t9214: f64) -> f64 {
    let t29833 = t1775 * t29832;
    let t29836 = t12234 * t29819;
    let t29837 = t1775 * t29836;
    let t29842 = 0.2698618307426597582e-1_f64 * t24976 - 0.14392630972941853771e0_f64 * t24978 + 0.26386490117060065246e0_f64 * t24980 - 0.53972366148531951639e-1_f64 * t2013 * t29812 + 0.35981577432354634428e-1_f64 * t2013 * t29816 - 0.35981577432354634427e-1_f64 * t2013 * t29821 + t12248 - 0.17990788716177317214e-1_f64 * t25007 + 0.53972366148531951639e-1_f64 * t2013 * t29826 - 0.53972366148531951639e-1_f64 * t7581 * t9214 - 0.2698618307426597582e-1_f64 * t2013 * t29833 + 0.53972366148531951639e-1_f64 * t2013 * t29837 - 0.53972366148531951639e-1_f64 * t25024 - 0.2698618307426597582e-1_f64 * t25027;
    t29842
}
