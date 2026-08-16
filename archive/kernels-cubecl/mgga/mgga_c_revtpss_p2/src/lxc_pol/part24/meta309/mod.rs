//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta309 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1095;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1096;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta309<F: Float>(t221: F, t3979: F, t6816: F, t3978: F, t3989: F, t6880: F, t22025: F, t543: F, t3992: F, t2661: F, t6836: F, t9921: F, t125: F, t6843: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t22056, t22057, t22059, t22061, t22062, t22063, t22068) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1095::<F>(t221, t3979, t6816, t3978, t3989, t6880, t22025, t543, t3992, t2661, t6836, t9921);
        let (t22069, t22074, t22079) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1096::<F>(t22068, t3978, t125, t6816, t6843);
    (t22056, t22057, t22059, t22061, t22062, t22063, t22068, t22069, t22074, t22079)
}
