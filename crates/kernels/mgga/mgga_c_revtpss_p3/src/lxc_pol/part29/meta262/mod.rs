//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta262 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1076;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1077;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1078;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1079;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1080;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta262<F: Float>(t1955: F, t860: F, t7056: F, t233: F, t2769: F, t822: F, t867: F, t30: F, t890: F, t33: F, t775: F, t1315: F, t196: F, t197: F, t1353: F, t1450: F, t533: F, t7021: F, t816: F, t1941: F, t540: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7067, t7070) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1076::<F>(t1955, t860, t7056);
        let t7071 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1077::<F>(t233, t2769);
        let t7076 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1078::<F>(t822, t867);
        let (t7092, t7200, t7207, t7234, t7235) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1079::<F>(t30, t890, t33, t775, t1315, t196, t197);
        let (t7238, t7250, t7252) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1080::<F>(t1353, t1450, t533, t7021, t816, t1941, t540);
    (t7067, t7070, t7071, t7076, t7092, t7200, t7207, t7234, t7235, t7238, t7250, t7252)
}
