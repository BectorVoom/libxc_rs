//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta544 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1856;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta544<F: Float>(t25365: F, t26506: F, t25305: F, t95540: F, t10115: F, t2063: F, t10982: F, t2061: F, t9646: F, t93190: F, t95726: F, t2435: F, t26560: F, t10073: F, t2066: F, t25390: F, t886: F, t7058: F, t95730: F, t2439: F, t26434: F, t887: F, t2471: F, t26563: F, t10985: F, t26576: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t95888, t95891, t95893, t95899, t95902, t95905) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1856::<F>(t25365, t26506, t25305, t95540, t10115, t2063, t10982, t2061, t9646, t93190, t95726, t2435, t26560);
        let (t95911, t95914, t95925, t95927, t95930) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1857::<F>(t10073, t2066, t25390, t886, t7058, t95730, t2439, t26434, t887, t2471, t26563, t10985, t26576);
    (t95888, t95891, t95893, t95899, t95902, t95905, t95911, t95914, t95925, t95927, t95930)
}
