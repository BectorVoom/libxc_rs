//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta309 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta309<F: Float>(t3555: F, t3754: F, t1248: F, t3153: F, t3566: F, t1269: F, t1284: F, t1209: F, t1204: F, t3781: F, t5462: F, t5477: F) -> (F, F, F, F, F, F, F) {
        let (t12709, t12712, t12717, t12723, t12744, t12751, t12756) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1078::<F>(t3555, t3754, t1248, t3153, t3566, t1269, t1284, t1209, t1204, t3781, t5462, t5477);
    (t12709, t12712, t12717, t12723, t12744, t12751, t12756)
}
