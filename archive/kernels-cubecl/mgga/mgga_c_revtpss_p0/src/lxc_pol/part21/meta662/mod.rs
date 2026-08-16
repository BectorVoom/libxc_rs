//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta662 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2457;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta662<F: Float>(t11626: F, t358: F, t3145: F, t3153: F, t3154: F, t11268: F, t3173: F, t1063: F, t11232: F, t3172: F, t11982: F, t11285: F, t3127: F) -> (F, F, F, F, F, F, F, F) {
        let (t42862, t42865, t42871, t42872, t42883, t42886, t42889, t42892) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2457::<F>(t11626, t358, t3145, t3153, t3154, t11268, t3173, t1063, t11232, t3172, t11982, t11285, t3127);
    (t42862, t42865, t42871, t42872, t42883, t42886, t42889, t42892)
}
