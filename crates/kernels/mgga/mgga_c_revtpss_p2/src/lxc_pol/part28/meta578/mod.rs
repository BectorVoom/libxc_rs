//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2042;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta578<F: Float>(t12020: F, t7121: F, t3223: F, t7131: F, t1033: F, t11266: F, t7120: F, t25526: F, t3173: F, t11263: F, t7122: F, t11762: F, t7111: F) -> (F, F, F, F, F, F) {
        let (t93761, t93764, t93774, t93799, t93801, t93813) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2042::<F>(t12020, t7121, t3223, t7131, t1033, t11266, t7120, t25526, t3173, t11263, t7122, t11762, t7111);
    (t93761, t93764, t93774, t93799, t93801, t93813)
}
