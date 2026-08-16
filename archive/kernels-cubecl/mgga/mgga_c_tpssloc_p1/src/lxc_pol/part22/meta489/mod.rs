//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1910;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta489<F: Float>(t21283: F, t21298: F, t913: F, t893: F, t10704: F, t21252: F, t10702: F, t1568: F, t17547: F, t1581: F, t5790: F, t1580: F, t17492: F) -> (F, F, F, F, F, F, F, F) {
        let (t21299, t21300, t21302, t21303, t21305, t21306, t21309, t21312) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1910::<F>(t21283, t21298, t913, t893, t10704, t21252, t10702, t1568, t17547, t1581, t5790, t1580, t17492);
    (t21299, t21300, t21302, t21303, t21305, t21306, t21309, t21312)
}
