//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta406 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1526;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1527;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1528;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1529;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1530;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1531;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1532;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta406<F: Float>(t2747: F, t2754: F, t4450: F, t4364: F, t4365: F, t231: F, t2394: F, t10770: F, t2719: F, t820: F, t844: F, t4368: F, t2724: F, t2482: F, t814: F, t14671: F, t14686: F, t4366: F, t10891: F, t10893: F, t10906: F, t14894: F, t14896: F, t14900: F, t14904: F, t14907: F, t2745: F, t4362: F, t14711: F, t14754: F, t14784: F, t14811: F, t14841: F, t14878: F, t14889: F, t136: F, t1568: F, t2457: F, t2710: F, t2470: F, t4522: F, t874: F, t10657: F, t10916: F, t10921: F, t14577: F, t14581: F, t14590: F, t14596: F, t14603: F, t14608: F, t14663: F, t1559: F, t213: F, t234: F, t2815: F, t4424: F, t4494: F, t4514: F, t879: F, t2718: F, t4469: F, t822: F, t10923: F, t10925: F, t10930: F, t10935: F, t10939: F, t10948: F, t10961: F, t10964: F, t10966: F, t10969: F, t10971: F, t10974: F, t14507: F, t2646: F, t4526: F, t837: F, t14540: F, t14572: F, t868: F, t4533: F, t72: F, t686: F, t2465: F, t1569: F, t867: F, t786: F, t2467: F, t122: F, t4480: F, t2466: F, t10995: F, t11044: F, t4481: F, t10498: F, t10501: F, t14474: F, t14479: F, t14484: F, t14486: F, t14489: F, t865: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14910, t14914, t14919, t14925) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1526::<F>(t2747, t2754, t4450, t4364, t4365, t231, t2394, t10770, t2719, t820, t844, t4368);
        let (t14927, t14933, t14936) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1527::<F>(t2724, t4364, t4365, t2482, t2719, t814, t14671, t14686, t4366, t10891, t10893, t10906, t14894, t14896, t14900, t14904, t14907, t14910, t14914, t14919, t14925, t2745, t4362);
        let (t14939, t14948) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1528::<F>(t14711, t14754, t14784, t14811, t14841, t14878, t14889, t14936, t136, t1568, t2457, t2710);
        let t14953 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1529::<F>(t2470, t4522, t874, t10657, t10916, t10921, t14577, t14581, t14590, t14596, t14603, t14608, t14663, t14939, t14948, t1559, t213, t234, t2754, t2815, t4424, t4494, t4514, t820, t879);
        let t14976 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1530::<F>(t1568, t2718, t4469, t822, t10923, t10925, t10930, t10935, t10939, t10948, t10961, t10964, t10966, t10969, t10971, t10974, t14507, t2646, t2724, t4514, t4526, t820, t837);
        let (t14978, t14979, t14983, t14985, t14987) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1531::<F>(t14540, t14572, t14953, t14976, t868, t4533, t72, t686, t2465, t1569, t867, t786);
        let (t14991, t14997) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1532::<F>(t14987, t2467, t122, t4480, t2466, t10995, t11044, t4481, t10498, t10501, t14474, t14479, t14484, t14486, t14489, t14979, t14985, t865);
    (t14910, t14914, t14919, t14927, t14933, t14939, t14978, t14979, t14983, t14991, t14997)
}
