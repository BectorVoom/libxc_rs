//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta187 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1138;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1139;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1140;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1141;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta187<F: Float>(t45: F, t57: F, t1469: F, t2375: F, t4186: F, t606: F, t78: F, t2382: F, t81: F, t162: F, t187: F, t150: F, t190: F, t1532: F, t750: F, zeta_threshold: F, t1534: F, t177: F, t762: F, t2611: F, t189: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4377, t4384, t4391, t4392, t4394, t4395, t4396, t4397) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1138::<F>(t45, t57, t1469, t2375, t4186, t606, t78, t2382, t81, t162, t187, t150, t190, t1532, t750, zeta_threshold);
        let t4398 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1139::<F>(t1534, t177);
        let (t4400, t4401) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1140::<F>(t4398, t762, t162, t2611);
        let t4402 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1141::<F>(t1469, t189);
    (t4377, t4384, t4391, t4392, t4394, t4395, t4396, t4397, t4398, t4400, t4401, t4402)
}
