//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta412 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1816;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1817;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta412<F: Float>(t10231: F, t4338: F, t973: F, t13542: F, t977: F, t10388: F, t10424: F, t10480: F, t10876: F, t10898: F, t10949: F, t13959: F, t13963: F, t13966: F, t13972: F, t13977: F, t13982: F, t13987: F, t13991: F, t13995: F, t1618: F, t3073: F, t3109: F, t3130: F, t4596: F, t4652: F, t13546: F, t13555: F, t2979: F, t13528: F, t13532: F, t10214: F, t13537: F, t13969: F, t4595: F, t1616: F, t2780: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13998, t14001, t14004) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1816::<F>(t10231, t4338, t973, t13542, t977, t10388, t10424, t10480, t10876, t10898, t10949, t13959, t13963, t13966, t13972, t13977, t13982, t13987, t13991, t13995, t1618, t3073, t3109, t3130, t4596, t4652);
        let (t14006, t14009, t14012, t14015, t14018, t14025, t14027, t14032) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1817::<F>(t13546, t977, t13555, t2979, t13528, t13532, t10214, t13537, t13969, t4595, t3130, t1616, t2780);
    (t13998, t14001, t14004, t14006, t14009, t14012, t14015, t14018, t14025, t14027, t14032)
}
