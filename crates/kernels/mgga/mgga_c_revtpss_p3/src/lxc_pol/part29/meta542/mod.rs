//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta542 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1877;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1878;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta542<F: Float>(t25365: F, t26544: F, t93190: F, t95726: F, t2435: F, t26560: F, t10073: F, t2066: F, t25390: F, t886: F, t7058: F, t95730: F, t2439: F, t26434: F, t887: F, t2471: F, t26563: F, t10985: F, t26576: F, t2062: F, t2769: F, t786: F, t10997: F, t26519: F, t93157: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t95900, t95902, t95905, t95911, t95914) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1877::<F>(t25365, t26544, t93190, t95726, t2435, t26560, t10073, t2066, t25390, t886, t7058, t95730);
        let (t95925, t95927, t95930, t95936, t95937, t95945) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1878::<F>(t2439, t26434, t887, t2471, t26563, t10985, t26576, t2062, t2769, t786, t10997, t26519, t93157);
    (t95900, t95902, t95905, t95911, t95914, t95925, t95927, t95930, t95936, t95937, t95945)
}
