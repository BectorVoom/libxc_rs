//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta668 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2468;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2469;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta668<F: Float>(t1086: F, t11200: F, t3090: F, t11671: F, t11926: F, t1045: F, t2862: F, t999: F, t3075: F, t606: F, t16565: F, t994: F, t42859: F, t42862: F, t342: F, t3145: F, t368: F) -> (F, F, F, F, F, F, F, F) {
        let (t43291, t43297, t43301, t43313, t43341) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2468::<F>(t1086, t11200, t3090, t11671, t11926, t1045, t2862, t999, t3075, t606, t16565, t994);
        let (t43346, t43347, t43350) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2469::<F>(t42859, t42862, t342, t3145, t368);
    (t43291, t43297, t43301, t43313, t43341, t43346, t43347, t43350)
}
