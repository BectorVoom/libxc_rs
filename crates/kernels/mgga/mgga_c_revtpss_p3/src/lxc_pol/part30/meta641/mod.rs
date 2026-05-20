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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta641<F: Float>(t17608: F, t7617: F, t17217: F, t26880: F, t17376: F, t26843: F, t26848: F, t29010: F, t3704: F, t17720: F, t7624: F, t1252: F, t17199: F, t17204: F, t17232: F, t17589: F, t3606: F, t3613: F, t97125: F, t15904: F, t26865: F, t13127: F, t17400: F, t26866: F, t1802: F, t3089: F, t3717: F, t13148: F, t17558: F, t17625: F, t17713: F, t17756: F, t17786: F, t29100: F, t3723: F, t97136: F, t97141: F, t97154: F, t97161: F, t97179: F, sigma2: F, t1285: F, t12987: F, t7623: F, t5261: F, t1230: F, t29082: F, t29037: F, t3636: F, t104647: F, t1266: F, t17265: F, t17347: F, t17369: F, t17732: F, t29040: F, t3631: F, t3640: F, t3644: F, t97169: F, t5326: F, t17544: F, t7618: F, t17523: F, t26842: F, t3594: F, t7616: F, t17373: F, t17769: F, t1797: F, t26873: F, t3591: F, t3714: F, t5287: F, t97120: F, t97171: F, t97177: F, t13142: F, t17384: F, t26867: F, t17640: F, t17646: F, t17690: F, t17705: F, t17750: F, t17781: F, t26852: F, t29097: F, t5304: F, t5354: F, t5402: F, t97182: F, t97187: F, t97232: F, t26827: F, t5362: F, t17435: F, t7613: F, t3670: F, t8184: F, t12702: F, t12744: F, t17391: F, t17602: F, t17744: F, t26870: F, t29062: F, t29096: F, t3663: F, t3674: F, t5335: F, t5343: F, t5348: F, t97191: F, t17303: F, t3678: F, t17209: F, t29019: F, t3707: F, t5265: F, t15687: F, t3767: F, t3782: F, t1224: F, t139: F, t29047: F, t5052: F, t3698: F, t5047: F, t16720: F, t16725: F, t17355: F, t17420: F, t17658: F, t17669: F, t17724: F, t29054: F, t5407: F, t97204: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t104692 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2228::<F>(t17608, t7617, t17217, t26880, t17376, t26843, t26848, t29010, t3704, t17720, t7624, t1252, t17199, t17204, t17232, t17589, t3606, t3613, t97125);
        let (t104695, t104707, t104718) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2229::<F>(t15904, t26865, t13127, t17400, t26866, t1802, t3089, t3717, t13148, t17558, t17625, t17713, t17756, t17786, t29100, t3723, t7624, t97136, t97141, t97154, t97161, t97179, sigma2);
        let t104746 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2230::<F>(t104707, t1285, t12987, t7623, t5261, t1230, t29082, t29037, t3636, t104647, t1266, t17265, t17347, t17369, t17732, t29040, t3631, t3640, t3644, t7624, t97169);
        let (t104752, t104756, t104758, t104762, t104768, t104770) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2231::<F>(t5326, t7623, t17544, t7618, t17523, t26842, t3594, t7616, t17373, t29040, t17769, t7624);
        let t104772 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2232::<F>(t104752, t104756, t104758, t104762, t104768, t104770, t1797, t26873, t29010, t3591, t3606, t3613, t3714, t5287, t97120, t97171, t97177);
        let t104796 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2233::<F>(t104695, t13142, t17384, t26867, t17640, t17646, t17690, t17705, t17750, t17781, t26852, t29097, t29100, t5304, t5354, t5402, t97182, t97187, t97232);
        let t104821 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2234::<F>(t26827, t5362, t17435, t7613, t3670, t8184, t12702, t12744, t17391, t17602, t17744, t26870, t29062, t29096, t3663, t3674, t5335, t5343, t5348, t97182, t97191);
        let (t104825, t104828, t104833, t104834, t104844, t104852) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2235::<F>(t17303, t7613, t29062, t3678, t17209, t26880, t29019, t3707, t26873, t5265, t15687, t26865);
        let t104876 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2236::<F>(t104852, t3767, t3782, t1224, t139, t29047, t5052, t3698, t5047, t16720, t16725, t17355, t17420, t17658, t17669, t17724, t26867, t26870, t29054, t29097, t5407, t97204, t97232);
    (t104692, t104718, t104746, t104772, t104796, t104821, t104825, t104828, t104833, t104834, t104844, t104876)
}
