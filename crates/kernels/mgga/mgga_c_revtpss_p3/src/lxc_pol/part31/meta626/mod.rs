//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta626 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2078;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2079;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta626<F: Float>(t1096: F, t357: F, t1976: F, t4743: F, t27543: F, t342: F, t4778: F, t8521: F, t1078: F, t42859: F, t1983: F, t3143: F, t1032: F, t4930: F, t994: F, t15669: F, t1035: F, t25698: F, t93920: F, t1647: F, t7135: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t99566, t99629, t99666, t99675, t99682, t99684) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2078::<F>(t1096, t357, t1976, t4743, t27543, t342, t4778, t8521, t1078, t42859, t1983, t3143);
        let (t99708, t99709, t99721, t99743, t99824, t99881) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2079::<F>(t1032, t4930, t994, t15669, t1976, t1035, t1983, t99682, t25698, t93920, t1647, t7135);
    (t99566, t99629, t99666, t99675, t99682, t99684, t99708, t99709, t99721, t99743, t99824, t99881)
}
