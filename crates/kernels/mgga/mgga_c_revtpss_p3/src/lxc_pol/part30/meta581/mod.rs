//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2035;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta581<F: Float>(t94564: F, t9795: F, t2018: F, t40688: F, t46808: F, t7256: F, t9784: F, t1445: F, t2439: F, t25916: F, t1358: F, t212: F, t26034: F, t689: F) -> (F, F, F, F, F) {
        let (t94565, t94569, t94571, t94580, t94584) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2035::<F>(t94564, t9795, t2018, t40688, t46808, t7256, t9784, t1445, t2439, t25916, t1358, t212, t26034, t689);
    (t94565, t94569, t94571, t94580, t94584)
}
