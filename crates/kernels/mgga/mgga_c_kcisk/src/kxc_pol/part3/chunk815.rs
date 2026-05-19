//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 815/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk815<F: Float>(t12552: F, t841: F, t848: F, t2883: F, t813: F, t14: F, t2886: F, t31: F, t12514: F, t2917: F, t52: F, t12535: F, t2921: F) -> (F, F, F, F) {
    let t12554 = t841 * t12552 * t848;
    let t12558 = F::new(1.0) / t2883 / t813;
    let t12559 = t14 * t12558;
    let t12561 = F::new(1.0) / t2886 / t31;
    let t12562 = t12514 * t12561;
    let t12564 = F::cast_from(0.51725014705706168417e3_f64) * t12559 * t12562;
    let t12566 = F::new(1.0) / t2917 / t52;
    let t12568 = t12566 * t12535 * t2921;
    (t12554, t12564, t12566, t12568)
}
