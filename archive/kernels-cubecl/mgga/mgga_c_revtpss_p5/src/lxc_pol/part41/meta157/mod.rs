//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta157 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk692;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk693;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta157<F: Float>(t45: F, t1522: F, t2398: F, t1568: F, t212: F, t780: F, t689: F, t1569: F, t786: F, t789: F, t1469: F, t80: F, t4186: F, t606: F, t766: F, zeta_threshold: F, t57: F, t83: F, t770: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4316, t4321, t4322, t4323, t4325, t4326, t4328, t4334) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk692::<F>(t45, t1522, t2398, t1568, t212, t780, t689, t1569, t786, t789, t1469, t80, t4186, t606, t766, zeta_threshold);
        let (t4335, t4343) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk693::<F>(t57, t1469, t83, t4186, t606, t770, t4334, zeta_threshold);
    (t4316, t4321, t4322, t4323, t4325, t4326, t4328, t4335, t4343)
}
