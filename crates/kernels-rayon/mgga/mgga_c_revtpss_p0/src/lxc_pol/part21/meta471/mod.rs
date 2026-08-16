//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2027;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2028;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2029;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2030;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2031;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta471(t14923: f64, t4368: f64, t2724: f64, t4364: f64, t4365: f64, t2482: f64, t2719: f64, t814: f64, t14671: f64, t14686: f64, t4366: f64, t10891: f64, t10893: f64, t10906: f64, t14894: f64, t14896: f64, t14900: f64, t14904: f64, t14907: f64, t14910: f64, t14914: f64, t14919: f64, t2745: f64, t4362: f64, t14711: f64, t14754: f64, t14784: f64, t14811: f64, t14841: f64, t14878: f64, t14889: f64, t136: f64, t1568: f64, t2457: f64, t2710: f64, t2470: f64, t4522: f64, t874: f64, t10657: f64, t10916: f64, t10921: f64, t14577: f64, t14581: f64, t14590: f64, t14596: f64, t14603: f64, t14608: f64, t14663: f64, t1559: f64, t213: f64, t234: f64, t2754: f64, t2815: f64, t4424: f64, t4494: f64, t4514: f64, t820: f64, t879: f64, t2718: f64, t4469: f64, t822: f64, t10923: f64, t10925: f64, t10930: f64, t10935: f64, t10939: f64, t10948: f64, t10961: f64, t10964: f64, t10966: f64, t10969: f64, t10971: f64, t10974: f64, t14507: f64, t2646: f64, t4526: f64, t837: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14925, t14927, t14931) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2027(t14923, t4368, t2724, t4364, t4365, t2482, t2719, t814);
        let (t14933, t14936) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2028(t14671, t14686, t4366, t14931, t10891, t10893, t10906, t14894, t14896, t14900, t14904, t14907, t14910, t14914, t14919, t14925, t14927, t2745, t4362);
        let t14939 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2029(t14711, t14754, t14784, t14811, t14841, t14878, t14889, t14936);
        let (t14946, t14953) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2030(t136, t1568, t2457, t2710, t2470, t4522, t874, t10657, t10916, t10921, t14577, t14581, t14590, t14596, t14603, t14608, t14663, t14939, t1559, t213, t234, t2754, t2815, t4424, t4494, t4514, t820, t879);
        let (t14961, t14972, t14976) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2031(t1568, t2718, t4469, t822, t10923, t10925, t10930, t10935, t10939, t10948, t10961, t10964, t10966, t10969, t10971, t10974, t14507, t2646, t2724, t4514, t4526, t820, t837);
    (t14927, t14931, t14933, t14939, t14946, t14953, t14961, t14972, t14976)
}
