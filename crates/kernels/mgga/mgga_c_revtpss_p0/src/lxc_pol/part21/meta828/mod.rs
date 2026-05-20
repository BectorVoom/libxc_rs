//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta828 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3084;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3085;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3086;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3087;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3088;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3089;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3090;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3091;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta828<F: Float>(t12627: F, t1811: F, t12657: F, t1208: F, t17330: F, t487: F, t1269: F, t17306: F, t1209: F, t1270: F, t3566: F, t56183: F, t43830: F, t43832: F, t44307: F, t56151: F, t56155: F, t56159: F, t56163: F, t56167: F, t56174: F, t56176: F, t56181: F, t56185: F, t56187: F, t56189: F, t56194: F, t56198: F, t56203: F, t56207: F, t56209: F, t56228: F, t43858: F, t43865: F, t43883: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t56212: F, t56214: F, t56216: F, t56221: F, t56226: F, t56230: F, t56234: F, t56236: F, t56248: F, t56252: F, t56256: F, t459: F, t1215: F, t12630: F, t12641: F, t1271: F, t1274: F, t1277: F, t13173: F, t13174: F, t13182: F, t17331: F, t17964: F, t17968: F, t17975: F, t17986: F, t17988: F, t18084: F, t18090: F, t18103: F, t3552: F, t3556: F, t3561: F, t3567: F, t3568: F, t3569: F, t3572: F, t3729: F, t3732: F, t3738: F, t495: F, t5216: F, t5251: F, t5414: F, t5497: F, t56315: F, t1204: F, t5412: F, t17288: F, t3584: F, t5245: F, t1210: F, t1211: F, t12607: F, t12633: F, t12651: F, t12658: F, t12696: F, t1295: F, t13183: F, t17999: F, t18047: F, t18062: F, t18087: F, t1828: F, t1829: F, t3585: F, t3791: F, t45430: F, t45487: F, t45552: F, t5220: F, t5225: F, t5231: F, t5423: F, t12621: F, t1774: F, t1214: F, t16750: F, t12629: F, t3555: F, t3565: F, t5215: F, t12603: F, t12650: F, t12654: F, t12666: F, t12695: F, t1294: F, t13165: F, t13177: F, t17963: F, t17987: F, t18019: F, t18109: F, t21389: F, t3737: F, t45438: F, t45482: F, t5237: F, t5246: F, t5429: F, t12599: F, t12600: F, t12628: F, t12647: F, t12673: F, t17973: F, t17995: F, t18030: F, t18054: F, t18059: F, t18070: F, t18114: F, t3576: F, t3739: F, t3790: F, t45427: F, t45449: F, t5498: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t56393, t56396, t56412, t56413, t56416, t56419, t56432, t56447) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3084::<F>(t12627, t1811, t12657, t1208, t17330, t487, t1269, t17306, t1209, t1270, t3566, t56183);
        let t56456 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3085::<F>(t43830, t43832, t44307, t56151, t56155, t56159, t56163, t56167, t56174, t56176, t56181, t56185, t56187, t56189, t56194, t56198, t56203, t56207, t56209, t56447);
        let t56477 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3086::<F>(t56228, t43858, t43865, t43883, t43888, t43890, t43892, t43894, t43896, t56212, t56214, t56216, t56221, t56226, t56230, t56234, t56236, t56248, t56252, t56256);
        let (t56479, t56484) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3087::<F>(t459, t56456, t56477, t1215, t12630, t12641, t1271, t1274, t1277, t13173, t13174, t13182, t17331, t17964, t17968, t17975, t17986, t17988, t18084, t18090, t18103, t3552, t3556, t3561, t3567, t3568, t3569, t3572, t3729, t3732, t3738, t495, t5216, t5251, t5414, t5497, t56315, t56393, t56396, t56413, t56416, t56419, t56432);
        let (t56530, t56534) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3088::<F>(t17331, t487, t1204, t5412, t1811, t3552, t1269, t17288, t3584, t5245, t1210, t1211, t1215, t12607, t12633, t12651, t12658, t12696, t1274, t1277, t1295, t13183, t17999, t18047, t18062, t18084, t18087, t18103, t1828, t1829, t3556, t3567, t3572, t3585, t3791, t45430, t45487, t45552, t5220, t5225, t5231, t5251, t5423, t5497);
        let (t56543, t56555, t56561, t56570, t56575, t56587, t56588) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3089::<F>(t12621, t1774, t1214, t16750, t12629, t3555, t5412, t1269, t5216, t3565, t5215, t487);
        let t56593 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3090::<F>(t1210, t1211, t1215, t12603, t12650, t12654, t12666, t12695, t1274, t1294, t1295, t13165, t13177, t17963, t17968, t17986, t17987, t18019, t18109, t1828, t21389, t3556, t3561, t3567, t3569, t3572, t3737, t3738, t45438, t45482, t5231, t5237, t5245, t5246, t5429, t56543, t56555, t56561, t56570, t56575, t56588);
        let (t56620, t56642) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3091::<F>(t3566, t5412, t3568, t5245, t1210, t1211, t12599, t12600, t12628, t12633, t12647, t12654, t12658, t12673, t1274, t1277, t13165, t13174, t1774, t17973, t17987, t17995, t17999, t18030, t18054, t18059, t18070, t18087, t18114, t3556, t3569, t3576, t3737, t3739, t3790, t3791, t45427, t45449, t5220, t5237, t5429, t5497, t5498);
    (t56412, t56479, t56484, t56530, t56534, t56543, t56555, t56561, t56587, t56593, t56620, t56642)
}
