//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta118 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk653;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk654;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta118<F: Float>(t1568: F, t212: F, t780: F, t689: F, t1569: F, t786: F, t789: F, t1469: F, t80: F, t83: F, t1544: F, t221: F, t2675: F, t2674: F, t1558: F, t243: F, t231: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4321, t4322, t4323, t4325, t4326, t4328, t4335, t4349) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk653::<F>(t1568, t212, t780, t689, t1569, t786, t789, t1469, t80, t83, t1544, t221, t2675);
        let (t4350, t4352, t4353) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk654::<F>(t2674, t4349, t1558, t243, t231);
    (t4321, t4322, t4323, t4325, t4326, t4328, t4335, t4349, t4350, t4352, t4353)
}
