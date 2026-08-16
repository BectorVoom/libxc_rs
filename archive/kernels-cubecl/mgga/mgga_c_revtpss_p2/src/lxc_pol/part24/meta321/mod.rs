//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta321 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1112;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1113;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta321<F: Float>(t22671: F, t36: F, t70: F, t1486: F, t5826: F, t1470: F, t5854: F, t1469: F, t5819: F, t10355: F, t4201: F, t5825: F, t48: F, t477: F, t53: F, t10368: F, t4210: F, t60: F, t10379: F, t1480: F, t1483: F, t44: F, t56: F, t5843: F, t5848: F, t5851: F, t61: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22672, t22673, t22676, t22681, t22688) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1112::<F>(t22671, t36, t70, t1486, t5826, t1470, t5854, t1469, t5819);
        let (t22699, t22700, t22709, t22712, t22715, t22718) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1113::<F>(t10355, t22688, t4201, t5825, t22671, t48, t477, t53, t10368, t4210, t60, t10379, t1480, t1483, t44, t56, t5843, t5848, t5851, t61, sigma2);
    (t22672, t22673, t22676, t22681, t22688, t22699, t22700, t22709, t22712, t22715, t22718)
}
