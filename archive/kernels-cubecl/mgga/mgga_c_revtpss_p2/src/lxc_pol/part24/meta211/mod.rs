//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta211 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk951;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk952;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta211<F: Float>(t11015: F, t787: F, t781: F, t9292: F, t2410: F, t261: F, t3335: F, t389: F, t1077: F, t225: F, t268: F, t271: F, t7021: F, t159: F, t3181: F, t2851: F, t631: F, t45: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11017, t11040, t11064, t11108, t11119, t11121, t11132) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk951::<F>(t11015, t787, t781, t9292, t2410, t261, t3335, t389, t1077, t225, t268, t271, t7021);
        let (t11133, t11142, t11144) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk952::<F>(t11132, t159, t3181, t2851, t631);
        let (t11149, t11150) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk953::<F>(t2851, t45);
    (t11017, t11040, t11064, t11108, t11119, t11121, t11132, t11133, t11142, t11144, t11149, t11150)
}
