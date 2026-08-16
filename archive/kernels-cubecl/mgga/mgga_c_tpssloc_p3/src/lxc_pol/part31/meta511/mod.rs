//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1707;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta511<F: Float>(t5614: F, t6614: F, t5617: F, t815: F, t6605: F, t2628: F, t5585: F, t23146: F, t5593: F, t1894: F, t236: F, t5544: F) -> (F, F, F, F, F, F, F) {
        let (t28370, t28372, t28373, t28375, t28376, t28380, t28383) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1707::<F>(t5614, t6614, t5617, t815, t6605, t2628, t5585, t23146, t5593, t1894, t236, t5544);
    (t28370, t28372, t28373, t28375, t28376, t28380, t28383)
}
