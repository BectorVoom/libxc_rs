//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1512;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta326<F: Float>(t1788: F, t2221: F, t2223: F, t11987: F, t1408: F, t2: F, t3704: F, t12000: F, t1649: F, t3711: F, t225: F, t5213: F) -> (F, F, F, F, F, F, F) {
        let (t15984, t15986, t15989, t15992, t16003, t16006, t16022) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1512::<F>(t1788, t2221, t2223, t11987, t1408, t2, t3704, t12000, t1649, t3711, t225, t5213);
    (t15984, t15986, t15989, t15992, t16003, t16006, t16022)
}
