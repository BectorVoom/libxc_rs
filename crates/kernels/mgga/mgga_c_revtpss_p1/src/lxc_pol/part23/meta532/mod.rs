//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta532<F: Float>(t19680: F, t70: F, t18281: F, t36: F, t5826: F, t627: F, t1486: F, t4181: F, t4187: F, t1470: F, t4217: F, t1494: F, t21686: F, t21687: F, t21690: F, t4182: F, t5820: F, t5827: F, t5830: F, t641: F, t85: F) -> (F, F, F, F, F, F, F, F) {
        let (t21695, t21698, t21699, t21702, t21707, t21710, t21713, t21720) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2060::<F>(t19680, t70, t18281, t36, t5826, t627, t1486, t4181, t4187, t1470, t4217, t1494, t21686, t21687, t21690, t4182, t5820, t5827, t5830, t641, t85);
    (t21695, t21698, t21699, t21702, t21707, t21710, t21713, t21720)
}
