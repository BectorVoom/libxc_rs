//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2207;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2208;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2209;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2210;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2211;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2212;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2213;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta623<F: Float>(t1977: F, t3057: F, t1078: F, t11200: F, t7143: F, t1651: F, t988: F, t15827: F, t27536: F, t15904: F, t25515: F, t12047: F, t15731: F, t7122: F, t15938: F, t16017: F, t16070: F, t16144: F, t16196: F, t16210: F, t1671: F, t1675: F, t25522: F, t27498: F, t4912: F, t7132: F, t93541: F, t93561: F, t93649: F, t93670: F, t25512: F, t4820: F, t370: F, t16087: F, t4890: F, t93595: F, t16055: F, t27493: F, t15925: F, t25516: F, t1087: F, t93751: F, t12116: F, t12160: F, t15703: F, t16022: F, t16091: F, t16205: F, t27492: F, t3120: F, t3299: F, t4896: F, t4902: F, t93555: F, t93564: F, t25526: F, t15769: F, t15687: F, t3317: F, t15693: F, t16172: F, t4869: F, t93570: F, t93573: F, t93579: F, t93583: F, t93585: F, t93761: F, t93774: F, t15822: F, t25508: F, t25525: F, t4878: F, t27450: F, t3173: F, t1047: F, t15782: F, t15791: F, t15834: F, t15952: F, t16140: F, t16149: F, t16167: F, t3164: F, t4825: F, t4875: F, t93646: F, t93764: F, t16035: F, t25580: F, t25569: F, t4817: F, t15592: F, t15622: F, t15847: F, t25517: F, t4783: F, t4831: F, t93543: F, t93597: F, t93602: F, t93611: F, t93616: F, t93667: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t99953, t99969, t99970, t99983, t99984, t99985) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2207::<F>(t1977, t3057, t1078, t11200, t7143, t1651, t988, t15827, t27536, t15904, t25515, t12047);
        let t100004 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2208::<F>(t15731, t7122, t15938, t16017, t16070, t16144, t16196, t16210, t1671, t1675, t25522, t27498, t4912, t7132, t93541, t93561, t93649, t93670, t99983, t99985);
        let (t100006, t100007, t100008, t100019, t100024, t100025, t100030) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2209::<F>(t25512, t4820, t25515, t370, t16087, t4890, t93595, t16055, t27493, t15925, t25516, t1087, t93751);
        let t100035 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2210::<F>(t100006, t100008, t100019, t100024, t100025, t100030, t12116, t12160, t15703, t16022, t16091, t16205, t27492, t27498, t3120, t3299, t4896, t4902, t7132, t93555, t93564);
        let (t100054, t100058) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2211::<F>(t25526, t4820, t15769, t25522, t15687, t25515, t3317, t15693, t16172, t1671, t25512, t4869, t93570, t93573, t93579, t93583, t93585, t93761, t93774);
        let t100085 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2212::<F>(t15822, t25508, t25525, t4878, t27450, t3173, t1047, t15782, t15791, t15834, t15952, t16140, t16149, t16167, t25522, t27493, t27536, t3164, t4825, t4875, t7132, t93646, t93764);
        let t100109 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2213::<F>(t16035, t25580, t25569, t4817, t100019, t15592, t15622, t15847, t25517, t3317, t4783, t4831, t4902, t4912, t7132, t93543, t93597, t93602, t93611, t93616, t93667);
    (t99953, t99969, t99970, t99984, t100004, t100007, t100030, t100035, t100054, t100058, t100085, t100109)
}
