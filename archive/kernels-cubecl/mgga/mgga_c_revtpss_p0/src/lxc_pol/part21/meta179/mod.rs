//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta179 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1113;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1114;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta179<F: Float>(t1466: F, t602: F, t1497: F, t644: F, t1469: F, t606: F, t30: F, t33: F, t70: F, t2255: F, zeta_threshold: F) -> (F, F, F, F, F) {
        let (t4173, t4178, t4181) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1113::<F>(t1466, t602, t1497, t644, t1469, t606);
        let (t4182, t4186) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1114::<F>(t30, t33, t4181, t70, t2255, zeta_threshold);
    (t4173, t4178, t4181, t4182, t4186)
}
