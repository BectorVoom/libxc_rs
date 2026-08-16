//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta122 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk659;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk660;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta122<F: Float>(t1549: F, t2703: F, t1568: F, t213: F, t1580: F, t779: F, t689: F, t1579: F, t72: F, t686: F, t2465: F, t1558: F, t251: F, t231: F, t2783: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4455, t4474, t4477, t4478, t4480, t4481, t4482, t4494) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk659::<F>(t1549, t2703, t1568, t213, t1580, t779, t689, t1579, t72, t686, t2465, t1558, t251);
        let t4496 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk660::<F>(t231, t2783, t4494);
    (t4455, t4474, t4477, t4478, t4480, t4481, t4482, t4494, t4496)
}
