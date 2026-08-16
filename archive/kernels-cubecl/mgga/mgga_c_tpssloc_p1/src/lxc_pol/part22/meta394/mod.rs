//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta394 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1685;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta394<F: Float>(t4781: F, t4785: F, t3313: F, t11277: F, t5988: F, t1117: F, t11275: F, t3411: F, t6106: F, t1157: F, t6105: F, t1164: F) -> (F, F, F, F, F, F, F, F) {
        let (t18262, t18264, t18265, t18266, t18268, t18270, t18271, t18273) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1685::<F>(t4781, t4785, t3313, t11277, t5988, t1117, t11275, t3411, t6106, t1157, t6105, t1164);
    (t18262, t18264, t18265, t18266, t18268, t18270, t18271, t18273)
}
