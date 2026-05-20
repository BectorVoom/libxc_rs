//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1834;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta489<F: Float>(t25997: F, t4021: F, t25273: F, t533: F, t816: F, t540: F, t7021: F, t1372: F, t3961: F, t7252: F, t1389: F, t7269: F) -> (F, F, F, F, F, F, F) {
        let (t25998, t26003, t26004, t26005, t26006, t26007, t26009) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1834::<F>(t25997, t4021, t25273, t533, t816, t540, t7021, t1372, t3961, t7252, t1389, t7269);
    (t25998, t26003, t26004, t26005, t26006, t26007, t26009)
}
