//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1995;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta546<F: Float>(t14749: F, t221: F, t14767: F, t4423: F, t836: F, t231: F, t50560: F, t18632: F, t2722: F, t50474: F, t14586: F, t2645: F) -> (F, F, F, F, F, F, F, F) {
        let (t50789, t50931, t51436, t51525, t51529, t51570, t51574, t51608) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1995::<F>(t14749, t221, t14767, t4423, t836, t231, t50560, t18632, t2722, t50474, t14586, t2645);
    (t50789, t50931, t51436, t51525, t51529, t51570, t51574, t51608)
}
