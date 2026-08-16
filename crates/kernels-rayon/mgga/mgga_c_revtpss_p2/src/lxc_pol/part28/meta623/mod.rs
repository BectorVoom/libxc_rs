//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2207;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2208;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2209;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2210;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2211;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2212;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2213;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta623(t1977: f64, t3057: f64, t1078: f64, t11200: f64, t7143: f64, t1651: f64, t988: f64, t15827: f64, t27536: f64, t15904: f64, t25515: f64, t12047: f64, t15731: f64, t7122: f64, t15938: f64, t16017: f64, t16070: f64, t16144: f64, t16196: f64, t16210: f64, t1671: f64, t1675: f64, t25522: f64, t27498: f64, t4912: f64, t7132: f64, t93541: f64, t93561: f64, t93649: f64, t93670: f64, t25512: f64, t4820: f64, t370: f64, t16087: f64, t4890: f64, t93595: f64, t16055: f64, t27493: f64, t15925: f64, t25516: f64, t1087: f64, t93751: f64, t12116: f64, t12160: f64, t15703: f64, t16022: f64, t16091: f64, t16205: f64, t27492: f64, t3120: f64, t3299: f64, t4896: f64, t4902: f64, t93555: f64, t93564: f64, t25526: f64, t15769: f64, t15687: f64, t3317: f64, t15693: f64, t16172: f64, t4869: f64, t93570: f64, t93573: f64, t93579: f64, t93583: f64, t93585: f64, t93761: f64, t93774: f64, t15822: f64, t25508: f64, t25525: f64, t4878: f64, t27450: f64, t3173: f64, t1047: f64, t15782: f64, t15791: f64, t15834: f64, t15952: f64, t16140: f64, t16149: f64, t16167: f64, t3164: f64, t4825: f64, t4875: f64, t93646: f64, t93764: f64, t16035: f64, t25580: f64, t25569: f64, t4817: f64, t15592: f64, t15622: f64, t15847: f64, t25517: f64, t4783: f64, t4831: f64, t93543: f64, t93597: f64, t93602: f64, t93611: f64, t93616: f64, t93667: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t99953, t99969, t99970, t99983, t99984, t99985) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2207(t1977, t3057, t1078, t11200, t7143, t1651, t988, t15827, t27536, t15904, t25515, t12047);
        let t100004 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2208(t15731, t7122, t15938, t16017, t16070, t16144, t16196, t16210, t1671, t1675, t25522, t27498, t4912, t7132, t93541, t93561, t93649, t93670, t99983, t99985);
        let (t100006, t100007, t100008, t100019, t100024, t100025, t100030) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2209(t25512, t4820, t25515, t370, t16087, t4890, t93595, t16055, t27493, t15925, t25516, t1087, t93751);
        let t100035 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2210(t100006, t100008, t100019, t100024, t100025, t100030, t12116, t12160, t15703, t16022, t16091, t16205, t27492, t27498, t3120, t3299, t4896, t4902, t7132, t93555, t93564);
        let (t100054, t100058) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2211(t25526, t4820, t15769, t25522, t15687, t25515, t3317, t15693, t16172, t1671, t25512, t4869, t93570, t93573, t93579, t93583, t93585, t93761, t93774);
        let t100085 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2212(t15822, t25508, t25525, t4878, t27450, t3173, t1047, t15782, t15791, t15834, t15952, t16140, t16149, t16167, t25522, t27493, t27536, t3164, t4825, t4875, t7132, t93646, t93764);
        let t100109 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2213(t16035, t25580, t25569, t4817, t100019, t15592, t15622, t15847, t25517, t3317, t4783, t4831, t4902, t4912, t7132, t93543, t93597, t93602, t93611, t93616, t93667);
    (t99953, t99969, t99970, t99984, t100004, t100007, t100030, t100035, t100054, t100058, t100085, t100109)
}
