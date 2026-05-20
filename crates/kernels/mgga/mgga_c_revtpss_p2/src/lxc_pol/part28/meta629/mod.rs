//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta629 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2265;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2266;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2267;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2268;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2269;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2270;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2271;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta629<F: Float>(t1936: F, t98484: F, t98487: F, t27123: F, t7002: F, t13514: F, t93: F, t101469: F, t1312: F, t28219: F, t25832: F, t7889: F, t10416: F, t7741: F, t13435: F, t2322: F, t28042: F, t13440: F, t5523: F, t101407: F, t97593: F, t25191: F, t7898: F, t1937: F, t49686: F, t75667: F, t13426: F, t6993: F, t101436: F, t101439: F, t101472: F, t101476: F, t101482: F, t101485: F, t101486: F, t101515: F, t1502: F, t1911: F, t2007: F, t25800: F, t25835: F, t27145: F, t27830: F, t28053: F, t3813: F, t569: F, t651: F, t670: F, t7725: F, t101432: F, t97635: F, t98422: F, t98468: F, t98512: F, t98563: F, t98612: F, t1913: F, t7337: F, t1916: F, t26120: F, t26127: F, t26130: F, t1459: F, t28265: F, t26124: F, t28264: F, t4292: F, t572: F, t7330: F, t1518: F, t2371: F, t4158: F, t7953: F, t117: F, t2327: F, t18204: F, t18208: F, t18214: F, t1918: F, t2040: F, t26106: F, t573: F, param_d: F, t28277: F, t28280: F, t5795: F, t7331: F, t28268: F, t116: F, t28276: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t101517, t101519, t101521, t101524, t101526, t101528, t101530) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2265::<F>(t1936, t98484, t98487, t27123, t7002, t13514, t93, t101469, t1312, t28219, t25832, t7889);
        let t101542 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2266::<F>(t10416, t7741, t13435, t2322, t28042, t13440, t5523, t101407, t101517, t101519, t101521, t101524, t101526, t101528, t101530, t97593);
        let t101555 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2267::<F>(t25191, t7898, t1937, t49686, t75667, t13426, t6993, t101436, t101439, t101472, t101476, t101482, t101485, t101486, t101515, t101542, t13514, t1502, t1911, t2007, t2322, t25800, t25835, t27145, t27830, t28053, t3813, t569, t651, t670, t7725);
        let (t101558, t101563, t101568) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2268::<F>(t101432, t101555, t97635, t98422, t98468, t98512, t98563, t98612, t1913, t7337, t1916, t26120);
        let (t101570, t101572, t101576, t101578, t101583, t101586, t101590) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2269::<F>(t1916, t26127, t26130, t1459, t28265, t26124, t28264, t4292, t572, t13514, t7330, t1518, t1936, t2371);
        let t101609 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2270::<F>(t1518, t572, t670, t7002, t4158, t7953, t101469, t117, t2327, t7741, t101558, t101568, t101570, t101572, t101576, t101578, t101583, t101586, t101590, t18204, t18208, t18214, t1918, t2040, t26106, t573, param_d);
        let (t101613, t101617, t101619, t101621, t101625, t101628) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2271::<F>(t1459, t28277, t28280, t5795, t7331, t28268, t116, t28042, t572, t670, t2371, t28276);
    (t101558, t101563, t101609, t101613, t101617, t101619, t101621, t101625, t101628)
}
