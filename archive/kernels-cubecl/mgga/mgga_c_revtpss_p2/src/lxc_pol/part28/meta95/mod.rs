//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta95 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk606;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk607;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk608;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk609;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk610;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk611;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk612;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk613;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk614;
use chunk9::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk615;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta95<F: Float>(t1941: F, t533: F, t816: F, t546: F, t64: F, t213: F, t552: F, t225: F, t561: F, t1955: F, t555: F, t1032: F, t1426: F, t545: F, t532: F, t1450: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2016, t2018) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk606::<F>(t1941, t533, t816, t546, t64);
        let (t2019, t2022) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk607::<F>(t2018, t213, t552, t2016);
        let t2023 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk608::<F>(t2022, t225);
        let (t2024, t2027) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk609::<F>(t2023, t561, t1955, t555);
        let t2028 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk610::<F>(t1032, t1426);
        let t2029 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk611::<F>(t2022, t545);
        let t2030 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk612::<F>(t2028, t2029);
        let t2033 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk613::<F>(t2024, t2027, t2030, t213);
        let t2034 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk614::<F>(t2033, t532);
        let t2035 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk615::<F>(t1450, t2034);
    (t2018, t2019, t2022, t2023, t2024, t2027, t2028, t2029, t2030, t2033, t2034, t2035)
}
