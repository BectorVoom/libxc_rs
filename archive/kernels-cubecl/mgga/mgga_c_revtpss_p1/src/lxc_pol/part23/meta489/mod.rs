//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1961;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1962;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta489<F: Float>(t1179: F, t1188: F, t20382: F, t1196: F, t5192: F, t5202: F, t5207: F, t1189: F, t6555: F, t5181: F, t5197: F, t16988: F, t5205: F, t300: F, t6513: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t20384, t20386, t20388, t20390, t20391, t20393, t20394, t20396, t20397) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1961::<F>(t1179, t1188, t20382, t1196, t5192, t5202, t5207, t1189, t6555, t5181, t5197, t16988, t5205);
        let (t20399, t20400) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1962::<F>(t1196, t20397, t300, t6513);
    (t20384, t20386, t20388, t20390, t20391, t20393, t20394, t20396, t20397, t20399, t20400)
}
