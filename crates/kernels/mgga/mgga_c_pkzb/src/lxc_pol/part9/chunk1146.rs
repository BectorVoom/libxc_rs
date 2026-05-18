//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1146/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1146<F: Float>(t19845: F, t19865: F, t184: F, t5418: F, t16388: F, t2583: F, t5221: F, t6916: F, t6920: F, t149: F, t5224: F, t63: F) -> (F, F, F, F, F, F) {
    let t19867 = t19845 / F::new(2.0) + t19865 / F::new(2.0);
    let t19873 = t184 * t5418;
    let t19909 = t16388 * t2583;
    let t19910 = F::new(35.0) / F::new(24.0) * t19909;
    let t19911 = t5221 * t6916;
    let t19913 = t5221 * t6920;
    let t19932 = t149 * t5224 * t63;
    (t19867, t19873, t19910, t19911, t19913, t19932)
}
