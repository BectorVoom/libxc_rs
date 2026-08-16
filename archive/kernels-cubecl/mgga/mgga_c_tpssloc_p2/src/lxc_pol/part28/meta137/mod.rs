//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta137 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk741;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk742;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta137<F: Float>(t3061: F, t61: F, t248: F, t2771: F, t363: F, t368: F, t1017: F, t67: F, t1058: F, t1044: F, t820: F) -> (F, F, F, F, F, F, F) {
        let (t3062, t3064, t3067, t3068, t3069, t3070) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk741::<F>(t3061, t61, t248, t2771, t363, t368, t1017, t67, t1058);
        let t3071 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk742::<F>(t1044, t820);
    (t3062, t3064, t3067, t3068, t3069, t3070, t3071)
}
