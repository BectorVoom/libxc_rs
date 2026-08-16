//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1734;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta420<F: Float>(t15453: F, t17686: F, t4582: F, t17635: F, t4972: F, t1090: F, t6230: F, t3578: F, t6219: F, t4997: F, t5002: F, t11784: F, t248: F, t5971: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18954, t18955, t18958, t18959, t18964, t18965, t18968, t18969, t18972, t18975) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1734::<F>(t15453, t17686, t4582, t17635, t4972, t1090, t6230, t3578, t6219, t4997, t5002, t11784, t248, t5971);
    (t18954, t18955, t18958, t18959, t18964, t18965, t18968, t18969, t18972, t18975)
}
