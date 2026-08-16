//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta629 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2265;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2266;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2267;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2268;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2269;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2270;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2271;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta629(t1936: f64, t98484: f64, t98487: f64, t27123: f64, t7002: f64, t13514: f64, t93: f64, t101469: f64, t1312: f64, t28219: f64, t25832: f64, t7889: f64, t10416: f64, t7741: f64, t13435: f64, t2322: f64, t28042: f64, t13440: f64, t5523: f64, t101407: f64, t97593: f64, t25191: f64, t7898: f64, t1937: f64, t49686: f64, t75667: f64, t13426: f64, t6993: f64, t101436: f64, t101439: f64, t101472: f64, t101476: f64, t101482: f64, t101485: f64, t101486: f64, t101515: f64, t1502: f64, t1911: f64, t2007: f64, t25800: f64, t25835: f64, t27145: f64, t27830: f64, t28053: f64, t3813: f64, t569: f64, t651: f64, t670: f64, t7725: f64, t101432: f64, t97635: f64, t98422: f64, t98468: f64, t98512: f64, t98563: f64, t98612: f64, t1913: f64, t7337: f64, t1916: f64, t26120: f64, t26127: f64, t26130: f64, t1459: f64, t28265: f64, t26124: f64, t28264: f64, t4292: f64, t572: f64, t7330: f64, t1518: f64, t2371: f64, t4158: f64, t7953: f64, t117: f64, t2327: f64, t18204: f64, t18208: f64, t18214: f64, t1918: f64, t2040: f64, t26106: f64, t573: f64, param_d: f64, t28277: f64, t28280: f64, t5795: f64, t7331: f64, t28268: f64, t116: f64, t28276: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t101517, t101519, t101521, t101524, t101526, t101528, t101530) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2265(t1936, t98484, t98487, t27123, t7002, t13514, t93, t101469, t1312, t28219, t25832, t7889);
        let t101542 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2266(t10416, t7741, t13435, t2322, t28042, t13440, t5523, t101407, t101517, t101519, t101521, t101524, t101526, t101528, t101530, t97593);
        let t101555 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2267(t25191, t7898, t1937, t49686, t75667, t13426, t6993, t101436, t101439, t101472, t101476, t101482, t101485, t101486, t101515, t101542, t13514, t1502, t1911, t2007, t2322, t25800, t25835, t27145, t27830, t28053, t3813, t569, t651, t670, t7725);
        let (t101558, t101563, t101568) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2268(t101432, t101555, t97635, t98422, t98468, t98512, t98563, t98612, t1913, t7337, t1916, t26120);
        let (t101570, t101572, t101576, t101578, t101583, t101586, t101590) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2269(t1916, t26127, t26130, t1459, t28265, t26124, t28264, t4292, t572, t13514, t7330, t1518, t1936, t2371);
        let t101609 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2270(t1518, t572, t670, t7002, t4158, t7953, t101469, t117, t2327, t7741, t101558, t101568, t101570, t101572, t101576, t101578, t101583, t101586, t101590, t18204, t18208, t18214, t1918, t2040, t26106, t573, param_d);
        let (t101613, t101617, t101619, t101621, t101625, t101628) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2271(t1459, t28277, t28280, t5795, t7331, t28268, t116, t28042, t572, t670, t2371, t28276);
    (t101558, t101563, t101609, t101613, t101617, t101619, t101621, t101625, t101628)
}
