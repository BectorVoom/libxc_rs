//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2046;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2047;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta588<F: Float>(t136: F, t2457: F, t7307: F, t25944: F, t26035: F, t686: F, t72: F, t7284: F, t25878: F, t94597: F, t10073: F, t25937: F, t7274: F, t7282: F, t1955: F, t9656: F, t25904: F, t94634: F, t94640: F, t281: F, t555: F, t93238: F, t25898: F, t7303: F, t25917: F, t9303: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94806, t94807, t94810, t94811, t94813, t94820) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2046::<F>(t136, t2457, t7307, t25944, t26035, t686, t72, t7284, t25878, t94597, t10073, t25937, t7274, t7282);
        let (t94823, t94842, t94844, t94849, t94851, t94854) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2047::<F>(t1955, t7282, t9656, t25904, t94634, t94640, t281, t555, t93238, t25898, t7303, t25917, t9303);
    (t94806, t94807, t94810, t94811, t94813, t94820, t94823, t94842, t94844, t94849, t94851, t94854)
}
