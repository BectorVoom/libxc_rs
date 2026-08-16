//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1732;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1733;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta378<F: Float>(t13123: F, t2375: F, t184: F, t3966: F, t607: F, t4194: F, t12606: F, t185: F, t707: F, t4094: F, t706: F, t708: F, t9924: F, t9933: F, t13112: F, t13114: F, t13117: F, t13118: F, t13121: F, t13122: F, t9853: F, t9859: F, t9907: F, t9921: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13125, t13126, t13127, t13129, t13130, t13132, t13133, t13135) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1732::<F>(t13123, t2375, t184, t3966, t607, t4194, t12606, t185, t707, t4094, t706, t708);
        let (t13136, t13137, t13138) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1733::<F>(t9924, t9933, t13112, t13114, t13117, t13118, t13121, t13122, t13125, t13129, t13132, t13135, t9853, t9859, t9907, t9921);
    (t13125, t13126, t13127, t13129, t13130, t13132, t13133, t13135, t13136, t13137, t13138)
}
