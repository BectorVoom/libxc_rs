//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2247;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2248;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta599<F: Float>(t1045: F, t23997: F, t3117: F, t1651: F, t6305: F, t3155: F, t3162: F, t11765: F, t22688: F, t1012: F, t23598: F, t373: F, t371: F, t372: F, t6244: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23998, t23999, t24007) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2247::<F>(t1045, t23997, t3117, t1651, t6305);
        let (t24008, t24009, t24012, t24013, t24016, t24017, t24022, t24024, t24031) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2248::<F>(t24007, t3155, t3117, t3162, t11765, t22688, t1012, t23598, t373, t371, t372, t1651, t6244);
    (t23998, t23999, t24007, t24008, t24009, t24012, t24013, t24016, t24017, t24022, t24024, t24031)
}
