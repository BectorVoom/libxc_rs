//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta519 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1841;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta519<F: Float>(t2453: F, t555: F, t25898: F, t1399: F, t2438: F, t25304: F, t1444: F, t543: F, t268: F, t4102: F, t4057: F, t676: F) -> (F, F, F, F, F, F, F) {
        let (t94382, t94383, t94386, t94390, t94391, t94398, t94403) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1841::<F>(t2453, t555, t25898, t1399, t2438, t25304, t1444, t543, t268, t4102, t4057, t676);
    (t94382, t94383, t94386, t94390, t94391, t94398, t94403)
}
