//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta414 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1468;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1469;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1470;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta414<F: Float>(t4292: F, t94: F, t1513: F, t665: F, t93: F, t5920: F, t1501: F, t1518: F, t2339: F, t625: F, t655: F, t10208: F, t69: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t27126, t28036, t28219, t29508, t30138, t30143, t31027) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1468::<F>(t4292, t94, t1513, t665, t93, t5920, t1501, t1518, t2339, t625);
        let t31032 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1469::<F>(t625, t655);
        let t31035 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1470::<F>(t10208, t69);
    (t27126, t28036, t28219, t29508, t30138, t30143, t31027, t31032, t31035)
}
