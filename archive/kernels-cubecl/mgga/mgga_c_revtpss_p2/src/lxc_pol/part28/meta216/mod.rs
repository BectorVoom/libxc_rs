//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta216 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1021;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1022;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1023;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1024;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1025;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1026;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1027;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1028;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta216<F: Float>(t1063: F, t4817: F, t1670: F, t3172: F, t1041: F, t1065: F, t1651: F, t906: F, t1042: F, t1066: F, t4583: F, t247: F, t1062: F, t1659: F, t3204: F, t3116: F, t4757: F, t127: F, t1663: F, t371: F, t1025: F, t1068: F, t1675: F, t3106: F, t3112: F, t3127: F, t3174: F, t3188: F, t373: F, t4772: F, t372: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4818, t4820) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1021::<F>(t1063, t4817, t1670, t3172);
        let (t4821, t4823, t4824, t4825) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1022::<F>(t1041, t4820, t1065, t1651, t906, t1042);
        let t4831 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1023::<F>(t1066, t4583, t247);
        let t4834 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1024::<F>(t1062, t1659);
        let (t4837, t4839) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1025::<F>(t1062, t3204, t3116, t4757, t247);
        let t4845 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1026::<F>(t127, t1663, t371);
        let t4848 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1027::<F>(t1025, t4845, t1063, t1068, t1675, t3106, t3112, t3127, t3174, t3188, t4818, t4821, t4825, t4831, t4834, t4837, t4839);
        let (t4852, t4854) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1028::<F>(t373, t4772, t371, t372);
    (t4820, t4823, t4824, t4825, t4831, t4834, t4837, t4839, t4845, t4848, t4852, t4854)
}
