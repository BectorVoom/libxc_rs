//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1468;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta302<F: Float>(t1022: F, t1615: F, t360: F, t883: F, t607: F, t13566: F, t13602: F, t1573: F, t2904: F, t4408: F, t923: F, t1561: F, t2885: F) -> (F, F, F, F, F, F, F, F) {
        let (t14218, t14219, t14220, t14245, t14246, t14263, t14266, t14271) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1468::<F>(t1022, t1615, t360, t883, t607, t13566, t13602, t1573, t2904, t4408, t923, t1561, t2885);
    (t14218, t14219, t14220, t14245, t14246, t14263, t14266, t14271)
}
