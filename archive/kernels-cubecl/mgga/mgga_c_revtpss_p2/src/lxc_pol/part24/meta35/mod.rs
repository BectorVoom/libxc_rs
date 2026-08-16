//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta35 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk256;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk257;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk258;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk259;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta35<F: Float>(t760: F, t762: F, t206: F, t262: F, t78: F, t81: F, t212: F, t251: F, t225: F, t257: F, t689: F, t211: F, t209: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t764, t765, t766, t770, t779) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk256::<F>(t760, t762, t206, t262, t78, t81, t212, t251);
        let t780 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk257::<F>(t225, t257);
        let (t781, t783, t784, t785) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk258::<F>(t779, t780, t689, t211);
        let t786 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk259::<F>(t209, t785);
    (t764, t765, t766, t770, t779, t780, t781, t783, t784, t785, t786)
}
