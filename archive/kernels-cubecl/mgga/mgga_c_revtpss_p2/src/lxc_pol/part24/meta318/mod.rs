//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1105;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta318<F: Float>(t22453: F, t9680: F, t4147: F, t6781: F, t6922: F, t566: F, t6816: F, t1843: F, t5920: F, t1513: F, t5891: F, t10208: F) -> (F, F, F, F, F, F) {
        let (t22454, t22466, t22483, t22486, t22578, t22590) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1105::<F>(t22453, t9680, t4147, t6781, t6922, t566, t6816, t1843, t5920, t1513, t5891, t10208);
    (t22454, t22466, t22483, t22486, t22578, t22590)
}
