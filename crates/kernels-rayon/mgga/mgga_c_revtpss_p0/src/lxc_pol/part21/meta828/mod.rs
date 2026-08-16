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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3084;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3085;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3086;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3087;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3088;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3089;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3090;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3091;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta828(t12627: f64, t1811: f64, t12657: f64, t1208: f64, t17330: f64, t487: f64, t1269: f64, t17306: f64, t1209: f64, t1270: f64, t3566: f64, t56183: f64, t43830: f64, t43832: f64, t44307: f64, t56151: f64, t56155: f64, t56159: f64, t56163: f64, t56167: f64, t56174: f64, t56176: f64, t56181: f64, t56185: f64, t56187: f64, t56189: f64, t56194: f64, t56198: f64, t56203: f64, t56207: f64, t56209: f64, t56228: f64, t43858: f64, t43865: f64, t43883: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t56212: f64, t56214: f64, t56216: f64, t56221: f64, t56226: f64, t56230: f64, t56234: f64, t56236: f64, t56248: f64, t56252: f64, t56256: f64, t459: f64, t1215: f64, t12630: f64, t12641: f64, t1271: f64, t1274: f64, t1277: f64, t13173: f64, t13174: f64, t13182: f64, t17331: f64, t17964: f64, t17968: f64, t17975: f64, t17986: f64, t17988: f64, t18084: f64, t18090: f64, t18103: f64, t3552: f64, t3556: f64, t3561: f64, t3567: f64, t3568: f64, t3569: f64, t3572: f64, t3729: f64, t3732: f64, t3738: f64, t495: f64, t5216: f64, t5251: f64, t5414: f64, t5497: f64, t56315: f64, t1204: f64, t5412: f64, t17288: f64, t3584: f64, t5245: f64, t1210: f64, t1211: f64, t12607: f64, t12633: f64, t12651: f64, t12658: f64, t12696: f64, t1295: f64, t13183: f64, t17999: f64, t18047: f64, t18062: f64, t18087: f64, t1828: f64, t1829: f64, t3585: f64, t3791: f64, t45430: f64, t45487: f64, t45552: f64, t5220: f64, t5225: f64, t5231: f64, t5423: f64, t12621: f64, t1774: f64, t1214: f64, t16750: f64, t12629: f64, t3555: f64, t3565: f64, t5215: f64, t12603: f64, t12650: f64, t12654: f64, t12666: f64, t12695: f64, t1294: f64, t13165: f64, t13177: f64, t17963: f64, t17987: f64, t18019: f64, t18109: f64, t21389: f64, t3737: f64, t45438: f64, t45482: f64, t5237: f64, t5246: f64, t5429: f64, t12599: f64, t12600: f64, t12628: f64, t12647: f64, t12673: f64, t17973: f64, t17995: f64, t18030: f64, t18054: f64, t18059: f64, t18070: f64, t18114: f64, t3576: f64, t3739: f64, t3790: f64, t45427: f64, t45449: f64, t5498: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56393, t56396, t56412, t56413, t56416, t56419, t56432, t56447) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3084(t12627, t1811, t12657, t1208, t17330, t487, t1269, t17306, t1209, t1270, t3566, t56183);
        let t56456 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3085(t43830, t43832, t44307, t56151, t56155, t56159, t56163, t56167, t56174, t56176, t56181, t56185, t56187, t56189, t56194, t56198, t56203, t56207, t56209, t56447);
        let t56477 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3086(t56228, t43858, t43865, t43883, t43888, t43890, t43892, t43894, t43896, t56212, t56214, t56216, t56221, t56226, t56230, t56234, t56236, t56248, t56252, t56256);
        let (t56479, t56484) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3087(t459, t56456, t56477, t1215, t12630, t12641, t1271, t1274, t1277, t13173, t13174, t13182, t17331, t17964, t17968, t17975, t17986, t17988, t18084, t18090, t18103, t3552, t3556, t3561, t3567, t3568, t3569, t3572, t3729, t3732, t3738, t495, t5216, t5251, t5414, t5497, t56315, t56393, t56396, t56413, t56416, t56419, t56432);
        let (t56530, t56534) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3088(t17331, t487, t1204, t5412, t1811, t3552, t1269, t17288, t3584, t5245, t1210, t1211, t1215, t12607, t12633, t12651, t12658, t12696, t1274, t1277, t1295, t13183, t17999, t18047, t18062, t18084, t18087, t18103, t1828, t1829, t3556, t3567, t3572, t3585, t3791, t45430, t45487, t45552, t5220, t5225, t5231, t5251, t5423, t5497);
        let (t56543, t56555, t56561, t56570, t56575, t56587, t56588) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3089(t12621, t1774, t1214, t16750, t12629, t3555, t5412, t1269, t5216, t3565, t5215, t487);
        let t56593 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3090(t1210, t1211, t1215, t12603, t12650, t12654, t12666, t12695, t1274, t1294, t1295, t13165, t13177, t17963, t17968, t17986, t17987, t18019, t18109, t1828, t21389, t3556, t3561, t3567, t3569, t3572, t3737, t3738, t45438, t45482, t5231, t5237, t5245, t5246, t5429, t56543, t56555, t56561, t56570, t56575, t56588);
        let (t56620, t56642) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3091(t3566, t5412, t3568, t5245, t1210, t1211, t12599, t12600, t12628, t12633, t12647, t12654, t12658, t12673, t1274, t1277, t13165, t13174, t1774, t17973, t17987, t17995, t17999, t18030, t18054, t18059, t18070, t18087, t18114, t3556, t3569, t3576, t3737, t3739, t3790, t3791, t45427, t45449, t5220, t5237, t5429, t5497, t5498);
    (t56412, t56479, t56484, t56530, t56534, t56543, t56555, t56561, t56587, t56593, t56620, t56642)
}
