//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta289 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1176;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta289<F: Float>(t4077: F, t676: F, t123: F, t9680: F, t1444: F, t2434: F, t3915: F, t1359: F, t9292: F, t1363: F, t9288: F, t1362: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t9681, t9682, t9683, t9685, t9686, t9687, t9691, t9692, t9694) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1176::<F>(t4077, t676, t123, t9680, t1444, t2434, t3915, t1359, t9292, t1363, t9288, t1362);
    (t9681, t9682, t9683, t9685, t9686, t9687, t9691, t9692, t9694)
}
