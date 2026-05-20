//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta53 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk319;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk320;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk321;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk322;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk323;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk324;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta53<F: Float>(t1032: F, t342: F, t358: F, t360: F, t336: F, t368: F, t365: F, t246: F, t372: F, t912: F, t938: F, t978: F, t980: F, t985: F, t373: F, t357: F, t73: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1033, t1034, t1035, t1036, t1038) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk319::<F>(t1032, t342, t358, t360, t336, t368);
        let t1040 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk320::<F>(t1038, t365, t1036);
        let t1041 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk321::<F>(t1033, t1040);
        let t1042 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk322::<F>(t246, t372);
        let t1043 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk323::<F>(t912, t938, t978, t980, t985);
        let (t1044, t1045) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk324::<F>(t1043, t373, t357, t73);
    (t1033, t1034, t1035, t1036, t1038, t1040, t1041, t1042, t1043, t1044, t1045)
}
