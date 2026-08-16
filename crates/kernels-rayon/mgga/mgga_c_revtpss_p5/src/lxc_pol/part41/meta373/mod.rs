//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta373 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1225;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1226;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1227;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1228;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1229;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1230;
use chunk6::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1231;
use chunk7::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1232;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta373(t6075: f64, t892: f64, t262: f64, t5962: f64, t10568: f64, t10577: f64, t10582: f64, t10584: f64, t10586: f64, t14353: f64, t14433: f64, t1544: f64, t18557: f64, t18558: f64, t18561: f64, t18564: f64, t18565: f64, t18567: f64, t2403: f64, t2404: f64, t4541: f64, t775: f64, t9514: f64, t9517: f64, t9521: f64, t2411: f64, t11064: f64, t6079: f64, t890: f64, t10592: f64, t10596: f64, t10604: f64, t10611: f64, t11088: f64, t14618: f64, t18571: f64, t18572: f64, t18573: f64, t18574: f64, t18578: f64, t18579: f64, t18581: f64, t18582: f64, t1940: f64, t198: f64, t4433: f64, t4546: f64, t4556: f64, t5966: f64, t9524: f64, t9542: f64, t18309: f64, t18848: f64, t1587: f64, t2: f64, t580: f64, t11506: f64, t6189: f64, t11509: f64, t972: f64, t981: f64, t11144: f64, t5819: f64, t606: f64, t11142: f64, t128: f64, t11150: f64, t2850: f64, t4186: f64, t4573: f64, t6093: f64, t689: f64, t6097: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t18864 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1225(t6075, t892, t262, t5962, t10568, t10577, t10582, t10584, t10586, t14353, t14433, t1544, t18557, t18558, t18561, t18564, t18565, t18567, t2403, t2404, t4541, t775, t9514, t9517, t9521);
        let t18882 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1226(t2411, t6075, t11064, t6079, t1544, t890, t10592, t10596, t10604, t10611, t11088, t14618, t18571, t18572, t18573, t18574, t18578, t18579, t18581, t18582, t1940, t198, t2403, t4433, t4541, t4546, t4556, t5966, t9524, t9542);
        let (t18884, t18892, t18902) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1227(t18309, t18848, t18864, t18882, t1587, t2, t580, t11506, t6189, t11509, t972, t981);
        let (t18904, t18906) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1228(t11144, t5819, t606, t11142, t128);
        let (t18909, t18911) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1229(t11150, t5819, t606, t2850, t128);
        let (t18913, t18915) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1230(t4186, t4573, t2850, t128);
        let t18919 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1231(t6093, t689);
        let t18924 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1232(t6097, t689);
    (t18884, t18892, t18902, t18904, t18906, t18909, t18911, t18913, t18915, t18919, t18924)
}
