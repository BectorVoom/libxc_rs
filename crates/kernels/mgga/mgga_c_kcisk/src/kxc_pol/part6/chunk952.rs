//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 952/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk952<F: Float>(t1775: F, t29832: F, t12234: F, t29819: F, t12248: F, t2013: F, t24976: F, t24978: F, t24980: F, t25007: F, t25024: F, t25027: F, t29812: F, t29816: F, t29821: F, t29826: F, t7581: F, t9214: F) -> F {
    let t29833 = t1775 * t29832;
    let t29836 = t12234 * t29819;
    let t29837 = t1775 * t29836;
    let t29842 = F::new(0.2698618307426597582e-1) * t24976 - F::new(0.14392630972941853771e0) * t24978 + F::new(0.26386490117060065246e0) * t24980 - F::new(0.53972366148531951639e-1) * t2013 * t29812 + F::new(0.35981577432354634428e-1) * t2013 * t29816 - F::new(0.35981577432354634427e-1) * t2013 * t29821 + t12248 - F::new(0.17990788716177317214e-1) * t25007 + F::new(0.53972366148531951639e-1) * t2013 * t29826 - F::new(0.53972366148531951639e-1) * t7581 * t9214 - F::new(0.2698618307426597582e-1) * t2013 * t29833 + F::new(0.53972366148531951639e-1) * t2013 * t29837 - F::new(0.53972366148531951639e-1) * t25024 - F::new(0.2698618307426597582e-1) * t25027;
    t29842
}
