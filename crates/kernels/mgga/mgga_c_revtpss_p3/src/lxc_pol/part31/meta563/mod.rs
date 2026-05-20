//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1974;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta563<F: Float>(t231: F, t61756: F, t1544: F, t2411: F, t22461: F, t4147: F, t6861: F, t9994: F, t1398: F, t221: F, t22274: F, t22279: F) -> (F, F, F, F, F, F, F) {
        let (t62695, t63185, t73407, t73820, t73842, t74419, t74423) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1974::<F>(t231, t61756, t1544, t2411, t22461, t4147, t6861, t9994, t1398, t221, t22274, t22279);
    (t62695, t63185, t73407, t73820, t73842, t74419, t74423)
}
