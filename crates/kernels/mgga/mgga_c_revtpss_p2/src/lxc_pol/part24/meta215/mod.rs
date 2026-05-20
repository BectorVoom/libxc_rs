//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta215 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk959;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk960;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta215<F: Float>(t2922: F, t913: F, t275: F, t290: F, t2925: F, t2966: F, t307: F, t302: F, t11132: F, t11337: F, t944: F, t2969: F, t310: F, t3010: F, t320: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11384, t11385, t11387, t11408, t11409, t11422, t11423, t11449, t11450, t11452) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk959::<F>(t2922, t913, t275, t290, t2925, t2966, t307, t302, t11132, t11337, t944, t2969, t310);
        let t11465 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk960::<F>(t3010, t320);
    (t11384, t11385, t11387, t11408, t11409, t11422, t11423, t11449, t11450, t11452, t11465)
}
