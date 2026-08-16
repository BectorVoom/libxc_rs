//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta487 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1780;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1781;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta487<F: Float>(t3167: F, t7120: F, t1033: F, t3173: F, t7122: F, t1007: F, t7106: F, t1968: F, t3080: F, t7105: F, t800: F, t3244: F, t7111: F, t3111: F, t7132: F, t1058: F, t7126: F, t1973: F, t3201: F, t7114: F, t1020: F, t7131: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25525, t25526, t25529, t25535, t25538, t25539) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1780::<F>(t3167, t7120, t1033, t3173, t7122, t1007, t7106, t1968, t3080, t7105, t800);
        let (t25543, t25551, t25557, t25560, t25564, t25569) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1781::<F>(t3244, t7111, t3111, t7132, t1058, t7126, t1973, t3201, t7114, t1020, t7131);
    (t25525, t25526, t25529, t25535, t25538, t25539, t25543, t25551, t25557, t25560, t25564, t25569)
}
