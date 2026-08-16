//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta177 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1162;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1163;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1164;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta177<F: Float>(t1469: F, t606: F, t30: F, t33: F, t70: F, t2255: F, zeta_threshold: F, t36: F, t1470: F, t627: F, t1486: F, t607: F, t2275: F, t48: F, t2282: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t4181 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1162::<F>(t1469, t606);
        let (t4182, t4186) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1163::<F>(t30, t33, t4181, t70, t2255, zeta_threshold);
        let (t4187, t4188, t4191, t4196, t4201, t4202, t4205, t4210) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1164::<F>(t36, t4186, t70, t1470, t627, t1486, t607, t1469, t2275, t606, t48, t2282);
    (t4181, t4182, t4186, t4187, t4188, t4191, t4196, t4201, t4202, t4205, t4210)
}
