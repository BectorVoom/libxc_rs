//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 920/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk920<F: Float>(t1248: F, t3579: F, t3979: F, t12952: F, t4065: F, t1249: F, t12957: F, t1311: F, t163: F, t3575: F, t24: F, t3951: F) -> (F, F, F, F, F) {
    let t13595 = t1248 * t3979 * t3579;
    let t13598 = t1248 * t4065 * t12952;
    let t13601 = t1248 * t1249 * t12957;
    let t13603 = t163 * t1311;
    let t13605 = t1248 * t13603 * t3575;
    let t13607 = t24 * t3951;
    (t13595, t13598, t13601, t13605, t13607)
}
