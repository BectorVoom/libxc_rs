//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2236;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta546<F: Float>(t18211: F, t4900: F, t15382: F, t15390: F, t1171: F, t6109: F, t6011: F, t699: F, t11219: F, t18206: F, t136: F, t3297: F) -> (F, F, F, F, F, F, F) {
        let (t18475, t18484, t18489, t18494, t18496, t18497, t18499) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2236::<F>(t18211, t4900, t15382, t15390, t1171, t6109, t6011, t699, t11219, t18206, t136, t3297);
    (t18475, t18484, t18489, t18494, t18496, t18497, t18499)
}
