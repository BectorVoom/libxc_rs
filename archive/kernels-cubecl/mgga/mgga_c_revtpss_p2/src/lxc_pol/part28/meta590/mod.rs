//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2060;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2061;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta590<F: Float>(t94570: F, t1445: F, t2439: F, t25916: F, t1358: F, t212: F, t26034: F, t689: F, t25877: F, t94390: F, t94385: F, t9675: F, t7289: F, t94377: F, t122: F, t72: F, t7274: F, t3916: F, t25895: F, t7285: F, t9288: F, t7284: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94571, t94580, t94584, t94589, t94590) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2060::<F>(t94570, t1445, t2439, t25916, t1358, t212, t26034, t689, t25877, t94390, t94385, t9675);
        let (t94591, t94593, t94596, t94597, t94598, t94600, t94602) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2061::<F>(t94589, t94590, t7289, t94377, t122, t72, t7274, t3916, t25895, t7285, t9288, t7284);
    (t94571, t94580, t94584, t94589, t94590, t94591, t94593, t94596, t94597, t94598, t94600, t94602)
}
