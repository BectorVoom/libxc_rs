//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta331 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1346;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta331<F: Float>(t11043: F, t786: F, t2467: F, t2828: F, t676: F, t123: F, t2465: F, t2410: F, t261: F) -> (F, F, F, F, F) {
        let (t11044, t11045, t11050, t11051, t11064) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1346::<F>(t11043, t786, t2467, t2828, t676, t123, t2465, t2410, t261);
    (t11044, t11045, t11050, t11051, t11064)
}
