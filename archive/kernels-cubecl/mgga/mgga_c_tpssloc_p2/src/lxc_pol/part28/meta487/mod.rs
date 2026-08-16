//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta487 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1701;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta487<F: Float>(t22792: F, t26271: F, t5227: F, t6916: F, t1998: F, t236: F, t5187: F, t6926: F, t1878: F, t22683: F, t221: F, t5308: F) -> (F, F, F, F, F, F) {
        let (t26272, t26274, t26277, t26278, t26284, t26285) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1701::<F>(t22792, t26271, t5227, t6916, t1998, t236, t5187, t6926, t1878, t22683, t221, t5308);
    (t26272, t26274, t26277, t26278, t26284, t26285)
}
