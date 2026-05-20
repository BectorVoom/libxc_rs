//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta577 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2041;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta577<F: Float>(t11782: F, t1972: F, t1007: F, t25532: F, t3080: F, t7106: F, t11735: F, t1968: F, t11772: F, t25515: F, t3114: F, t11923: F, t25580: F) -> (F, F, F, F, F, F, F) {
        let (t93736, t93743, t93745, t93750, t93751, t93752, t93755) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2041::<F>(t11782, t1972, t1007, t25532, t3080, t7106, t11735, t1968, t11772, t25515, t3114, t11923, t25580);
    (t93736, t93743, t93745, t93750, t93751, t93752, t93755)
}
