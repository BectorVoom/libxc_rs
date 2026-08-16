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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1225;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1226;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1227;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1228;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1229;
use chunk5::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1230;
use chunk6::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1231;
use chunk7::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1232;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta373<F: Float>(t6075: F, t892: F, t262: F, t5962: F, t10568: F, t10577: F, t10582: F, t10584: F, t10586: F, t14353: F, t14433: F, t1544: F, t18557: F, t18558: F, t18561: F, t18564: F, t18565: F, t18567: F, t2403: F, t2404: F, t4541: F, t775: F, t9514: F, t9517: F, t9521: F, t2411: F, t11064: F, t6079: F, t890: F, t10592: F, t10596: F, t10604: F, t10611: F, t11088: F, t14618: F, t18571: F, t18572: F, t18573: F, t18574: F, t18578: F, t18579: F, t18581: F, t18582: F, t1940: F, t198: F, t4433: F, t4546: F, t4556: F, t5966: F, t9524: F, t9542: F, t18309: F, t18848: F, t1587: F, t2: F, t580: F, t11506: F, t6189: F, t11509: F, t972: F, t981: F, t11144: F, t5819: F, t606: F, t11142: F, t128: F, t11150: F, t2850: F, t4186: F, t4573: F, t6093: F, t689: F, t6097: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t18864 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1225::<F>(t6075, t892, t262, t5962, t10568, t10577, t10582, t10584, t10586, t14353, t14433, t1544, t18557, t18558, t18561, t18564, t18565, t18567, t2403, t2404, t4541, t775, t9514, t9517, t9521);
        let t18882 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1226::<F>(t2411, t6075, t11064, t6079, t1544, t890, t10592, t10596, t10604, t10611, t11088, t14618, t18571, t18572, t18573, t18574, t18578, t18579, t18581, t18582, t1940, t198, t2403, t4433, t4541, t4546, t4556, t5966, t9524, t9542);
        let (t18884, t18892, t18902) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1227::<F>(t18309, t18848, t18864, t18882, t1587, t2, t580, t11506, t6189, t11509, t972, t981);
        let (t18904, t18906) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1228::<F>(t11144, t5819, t606, t11142, t128);
        let (t18909, t18911) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1229::<F>(t11150, t5819, t606, t2850, t128);
        let (t18913, t18915) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1230::<F>(t4186, t4573, t2850, t128);
        let t18919 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1231::<F>(t6093, t689);
        let t18924 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1232::<F>(t6097, t689);
    (t18884, t18892, t18902, t18904, t18906, t18909, t18911, t18913, t18915, t18919, t18924)
}
