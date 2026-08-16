//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta641 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2228;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2229;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2230;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2231;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2232;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2233;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2234;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2235;
use chunk8::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2236;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta641(t17608: f64, t7617: f64, t17217: f64, t26880: f64, t17376: f64, t26843: f64, t26848: f64, t29010: f64, t3704: f64, t17720: f64, t7624: f64, t1252: f64, t17199: f64, t17204: f64, t17232: f64, t17589: f64, t3606: f64, t3613: f64, t97125: f64, t15904: f64, t26865: f64, t13127: f64, t17400: f64, t26866: f64, t1802: f64, t3089: f64, t3717: f64, t13148: f64, t17558: f64, t17625: f64, t17713: f64, t17756: f64, t17786: f64, t29100: f64, t3723: f64, t97136: f64, t97141: f64, t97154: f64, t97161: f64, t97179: f64, sigma2: f64, t1285: f64, t12987: f64, t7623: f64, t5261: f64, t1230: f64, t29082: f64, t29037: f64, t3636: f64, t104647: f64, t1266: f64, t17265: f64, t17347: f64, t17369: f64, t17732: f64, t29040: f64, t3631: f64, t3640: f64, t3644: f64, t97169: f64, t5326: f64, t17544: f64, t7618: f64, t17523: f64, t26842: f64, t3594: f64, t7616: f64, t17373: f64, t17769: f64, t1797: f64, t26873: f64, t3591: f64, t3714: f64, t5287: f64, t97120: f64, t97171: f64, t97177: f64, t13142: f64, t17384: f64, t26867: f64, t17640: f64, t17646: f64, t17690: f64, t17705: f64, t17750: f64, t17781: f64, t26852: f64, t29097: f64, t5304: f64, t5354: f64, t5402: f64, t97182: f64, t97187: f64, t97232: f64, t26827: f64, t5362: f64, t17435: f64, t7613: f64, t3670: f64, t8184: f64, t12702: f64, t12744: f64, t17391: f64, t17602: f64, t17744: f64, t26870: f64, t29062: f64, t29096: f64, t3663: f64, t3674: f64, t5335: f64, t5343: f64, t5348: f64, t97191: f64, t17303: f64, t3678: f64, t17209: f64, t29019: f64, t3707: f64, t5265: f64, t15687: f64, t3767: f64, t3782: f64, t1224: f64, t139: f64, t29047: f64, t5052: f64, t3698: f64, t5047: f64, t16720: f64, t16725: f64, t17355: f64, t17420: f64, t17658: f64, t17669: f64, t17724: f64, t29054: f64, t5407: f64, t97204: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t104692 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2228(t17608, t7617, t17217, t26880, t17376, t26843, t26848, t29010, t3704, t17720, t7624, t1252, t17199, t17204, t17232, t17589, t3606, t3613, t97125);
        let (t104695, t104707, t104718) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2229(t15904, t26865, t13127, t17400, t26866, t1802, t3089, t3717, t13148, t17558, t17625, t17713, t17756, t17786, t29100, t3723, t7624, t97136, t97141, t97154, t97161, t97179, sigma2);
        let t104746 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2230(t104707, t1285, t12987, t7623, t5261, t1230, t29082, t29037, t3636, t104647, t1266, t17265, t17347, t17369, t17732, t29040, t3631, t3640, t3644, t7624, t97169);
        let (t104752, t104756, t104758, t104762, t104768, t104770) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2231(t5326, t7623, t17544, t7618, t17523, t26842, t3594, t7616, t17373, t29040, t17769, t7624);
        let t104772 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2232(t104752, t104756, t104758, t104762, t104768, t104770, t1797, t26873, t29010, t3591, t3606, t3613, t3714, t5287, t97120, t97171, t97177);
        let t104796 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2233(t104695, t13142, t17384, t26867, t17640, t17646, t17690, t17705, t17750, t17781, t26852, t29097, t29100, t5304, t5354, t5402, t97182, t97187, t97232);
        let t104821 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2234(t26827, t5362, t17435, t7613, t3670, t8184, t12702, t12744, t17391, t17602, t17744, t26870, t29062, t29096, t3663, t3674, t5335, t5343, t5348, t97182, t97191);
        let (t104825, t104828, t104833, t104834, t104844, t104852) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2235(t17303, t7613, t29062, t3678, t17209, t26880, t29019, t3707, t26873, t5265, t15687, t26865);
        let t104876 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2236(t104852, t3767, t3782, t1224, t139, t29047, t5052, t3698, t5047, t16720, t16725, t17355, t17420, t17658, t17669, t17724, t26867, t26870, t29054, t29097, t5407, t97204, t97232);
    (t104692, t104718, t104746, t104772, t104796, t104821, t104825, t104828, t104833, t104834, t104844, t104876)
}
