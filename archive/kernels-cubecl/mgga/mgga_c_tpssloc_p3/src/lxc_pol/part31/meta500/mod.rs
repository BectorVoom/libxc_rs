//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta500 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1696;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta500<F: Float>(t22633: F, t28131: F, t19743: F, t3792: F, t22897: F, t1992: F, t6347: F, t6968: F, t6637: F, t6888: F, t6330: F, t22685: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t28132, t28134, t28135, t28136, t28138, t28139, t28140, t28142, t28143, t28144) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1696::<F>(t22633, t28131, t19743, t3792, t22897, t1992, t6347, t6968, t6637, t6888, t6330, t22685);
    (t28132, t28134, t28135, t28136, t28138, t28139, t28140, t28142, t28143, t28144)
}
