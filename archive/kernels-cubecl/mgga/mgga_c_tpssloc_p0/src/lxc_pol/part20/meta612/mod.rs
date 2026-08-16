//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2200;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta612<F: Float>(t107: F, t9576: F, t2585: F, t667: F, t2281: F, t2333: F, t2359: F, t626: F, t9367: F, t9371: F, t9412: F, t106: F, t9364: F) -> (F, F, F, F, F, F, F, F) {
        let (t45421, t45422, t45424, t45426, t45428, t45430, t45432, t45435) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2200::<F>(t107, t9576, t2585, t667, t2281, t2333, t2359, t626, t9367, t9371, t9412, t106, t9364);
    (t45421, t45422, t45424, t45426, t45428, t45430, t45432, t45435)
}
