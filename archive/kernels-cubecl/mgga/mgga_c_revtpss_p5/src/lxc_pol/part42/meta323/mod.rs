//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta323 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1104;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1105;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1106;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1107;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta323<F: Float>(t14109: F, t3916: F, t9680: F, t1437: F, t1882: F, t2482: F, t4104: F, t10073: F, t5737: F, t1419: F, t4086: F, t543: F, t2782: F, t555: F, t5658: F, t4114: F, t122: F, t4003: F, t72: F, t1398: F, t676: F, t10069: F, t5710: F, t1432: F, t686: F, t136: F, t1892: F, t2457: F, t3964: F, t2435: F, t5760: F, t545: F, t869: F, t689: F, t225: F, t9990: F, t213: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14111, t14116, t14120, t14124) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1104::<F>(t14109, t3916, t9680, t1437, t1882, t2482, t4104, t10073, t5737, t1419, t4086, t543);
        let (t14126, t14131, t14141, t14143) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1105::<F>(t14124, t2782, t555, t5658, t4086, t543, t1882, t4114, t2482, t122, t4003, t72);
        let (t14146, t14149, t14158, t14159) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1106::<F>(t1398, t676, t14143, t14141, t10069, t5737, t5710, t72, t1432, t686, t136, t1892);
        let (t14161, t14166, t14191, t14193) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1107::<F>(t14159, t2457, t3964, t2435, t5760, t545, t5710, t869, t689, t225, t9990, t213);
    (t14111, t14116, t14120, t14126, t14131, t14146, t14149, t14158, t14161, t14166, t14191, t14193)
}
