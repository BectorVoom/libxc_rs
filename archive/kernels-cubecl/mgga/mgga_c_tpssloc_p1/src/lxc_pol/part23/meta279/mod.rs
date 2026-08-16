//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta279 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk973;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta279<F: Float>(t13109: F, t13113: F, t16702: F, t185: F, t20234: F, t9897: F, t1462: F, t16689: F, t13124: F, t16711: F, t9853: F, t9859: F, t9907: F, t9921: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t20822, t20823, t20824, t20825, t20827, t20829, t20830, t20831, t20832) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk973::<F>(t13109, t13113, t16702, t185, t20234, t9897, t1462, t16689, t13124, t16711, t9853, t9859, t9907, t9921);
    (t20822, t20823, t20824, t20825, t20827, t20829, t20830, t20831, t20832)
}
