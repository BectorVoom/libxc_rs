//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta322 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1598;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1599;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta322<F: Float>(t11028: F, t780: F, t2439: F, t10910: F, t225: F, t2772: F, t779: F, t689: F, t781: F, t9292: F, t861: F, t867: F, t786: F) -> (F, F, F, F, F, F, F, F) {
        let (t11029, t11030, t11032, t11036, t11037, t11040, t11043) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1598::<F>(t11028, t780, t2439, t10910, t225, t2772, t779, t689, t781, t9292, t861, t867);
        let t11044 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1599::<F>(t11043, t786);
    (t11029, t11030, t11032, t11036, t11037, t11040, t11043, t11044)
}
