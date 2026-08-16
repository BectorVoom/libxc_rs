//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta323 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1104;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1105;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1106;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1107;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta323(t14109: f64, t3916: f64, t9680: f64, t1437: f64, t1882: f64, t2482: f64, t4104: f64, t10073: f64, t5737: f64, t1419: f64, t4086: f64, t543: f64, t2782: f64, t555: f64, t5658: f64, t4114: f64, t122: f64, t4003: f64, t72: f64, t1398: f64, t676: f64, t10069: f64, t5710: f64, t1432: f64, t686: f64, t136: f64, t1892: f64, t2457: f64, t3964: f64, t2435: f64, t5760: f64, t545: f64, t869: f64, t689: f64, t225: f64, t9990: f64, t213: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14111, t14116, t14120, t14124) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1104(t14109, t3916, t9680, t1437, t1882, t2482, t4104, t10073, t5737, t1419, t4086, t543);
        let (t14126, t14131, t14141, t14143) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1105(t14124, t2782, t555, t5658, t4086, t543, t1882, t4114, t2482, t122, t4003, t72);
        let (t14146, t14149, t14158, t14159) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1106(t1398, t676, t14143, t14141, t10069, t5737, t5710, t72, t1432, t686, t136, t1892);
        let (t14161, t14166, t14191, t14193) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1107(t14159, t2457, t3964, t2435, t5760, t545, t5710, t869, t689, t225, t9990, t213);
    (t14111, t14116, t14120, t14126, t14131, t14146, t14149, t14158, t14161, t14166, t14191, t14193)
}
