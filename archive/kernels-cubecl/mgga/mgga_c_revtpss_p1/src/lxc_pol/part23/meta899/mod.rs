//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta899 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2859;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2860;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta899<F: Float>(t1469: F, t4401: F, t61266: F, t40067: F, t40072: F, t40167: F, t40171: F, t40184: F, t50874: F, t50884: F, t77020: F, t77021: F, t77023: F, t77024: F, t77025: F, t77026: F, t77027: F, t77028: F, t77029: F, t18305: F, t4186: F, t18576: F, t62291: F, t62302: F, t50892: F, t50893: F, t189: F, t22671: F, t606: F, t177: F, t23211: F, t762: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t77032, t77033) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2859::<F>(t1469, t4401, t61266, t40067, t40072, t40167, t40171, t40184, t50874, t50884, t77020, t77021, t77023, t77024, t77025, t77026, t77027, t77028, t77029);
        let (t77036, t77038, t77039, t77040, t77041, t77045, t77047) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2860::<F>(t18305, t4186, t4401, t18576, t62291, t62302, t50892, t50893, t189, t22671, t606, t177, t23211, t762);
    (t77032, t77033, t77036, t77038, t77039, t77040, t77041, t77045, t77047)
}
