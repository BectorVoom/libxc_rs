//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta436 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1643;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1644;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta436<F: Float>(t3498: F, t5205: F, t1196: F, t12485: F, t1756: F, t3524: F, t3531: F, t5198: F, t12361: F, t5068: F, t12243: F, t5109: F, t1149: F, t5105: F, t3384: F, t1733: F, t3427: F, t3385: F, t5108: F, t12248: F, t3435: F, t5104: F, t3433: F, t12230: F, t1732: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t16641, t16645, t16647, t16649, t16651) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1643::<F>(t3498, t5205, t1196, t12485, t1756, t3524, t3531, t5198, t12361, t5068, t12243, t5109);
        let (t16654, t16657, t16660, t16664, t16667, t16668) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1644::<F>(t1149, t5105, t3384, t1733, t3427, t3385, t5108, t12248, t3435, t5104, t3433, t12230, t1732);
    (t16641, t16645, t16647, t16649, t16651, t16654, t16657, t16660, t16664, t16667, t16668)
}
