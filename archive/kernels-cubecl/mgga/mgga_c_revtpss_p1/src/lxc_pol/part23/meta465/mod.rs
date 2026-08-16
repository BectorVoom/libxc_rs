//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1906;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1907;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta465<F: Float>(t19380: F, t373: F, t371: F, t372: F, t19463: F, t366: F, t3094: F, t4186: F, t4781: F, t3092: F, t4786: F, t6092: F, t11703: F, t11710: F, t6267: F, t3091: F, t4583: F, t4823: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t19768, t19770, t19773) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1906::<F>(t19380, t373, t371, t372, t19463, t366);
        let (t19776, t19777, t19778, t19781, t19782, t19785, t19786, t19791) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1907::<F>(t3094, t4186, t4781, t3092, t4786, t6092, t11703, t11710, t6267, t3091, t4583, t4823);
    (t19768, t19770, t19773, t19776, t19777, t19778, t19781, t19782, t19785, t19786, t19791)
}
