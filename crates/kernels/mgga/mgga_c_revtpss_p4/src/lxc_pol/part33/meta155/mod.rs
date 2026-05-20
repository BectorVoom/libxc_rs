//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta155 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk791;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk792;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk793;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta155<F: Float>(t4086: F, t555: F, t786: F, t1398: F, t675: F, t268: F, t543: F, t1419: F, t72: F, t1432: F, t686: F, t1433: F, t2470: F, t3999: F, t1385: F, t198: F, t531: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4100, t4101) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk791::<F>(t4086, t555, t786);
        let (t4102, t4104, t4105, t4107, t4109, t4113, t4114) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk792::<F>(t1398, t675, t268, t543, t4101, t1419, t72, t1432, t686, t1433, t2470, t3999, t555);
        let (t4118, t4139) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk793::<F>(t1385, t1419, t198, t531);
    (t4100, t4101, t4102, t4104, t4105, t4107, t4109, t4113, t4114, t4118, t4139)
}
