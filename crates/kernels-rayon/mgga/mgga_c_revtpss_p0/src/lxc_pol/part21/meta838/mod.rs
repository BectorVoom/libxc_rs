//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta838 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3139;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3140;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3141;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3142;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3143;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3144;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3145;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta838(t1196: f64, t12548: f64, t5197: f64, t16643: f64, t3531: f64, t16682: f64, t1732: f64, t3433: f64, t12411: f64, t12556: f64, t1756: f64, t43752: f64, t16831: f64, t300: f64, t1198: f64, t56176: f64, t56183: f64, t43830: f64, t43832: f64, t45000: f64, t56151: f64, t56155: f64, t56159: f64, t56163: f64, t56167: f64, t56174: f64, t56181: f64, t56185: f64, t56187: f64, t56189: f64, t56194: f64, t56198: f64, t56203: f64, t56207: f64, t56209: f64, t56228: f64, t43858: f64, t43865: f64, t43883: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t56212: f64, t56214: f64, t56216: f64, t56221: f64, t56226: f64, t56230: f64, t56234: f64, t56236: f64, t56248: f64, t56252: f64, t56256: f64, t422: f64, t1189: f64, t17150: f64, t3495: f64, t57820: f64, t57822: f64, t57825: f64, t57827: f64, t57829: f64, t57831: f64, t57833: f64, t57835: f64, t57837: f64, t57840: f64, t57842: f64, t57846: f64, t1168: f64, t12423: f64, t12429: f64, t12430: f64, t12486: f64, t12487: f64, t12504: f64, t12508: f64, t12511: f64, t16948: f64, t16959: f64, t17023: f64, t17032: f64, t17085: f64, t17086: f64, t1745: f64, t1757: f64, t3452: f64, t3477: f64, t3479: f64, t45075: f64, t45188: f64, t45190: f64, t45194: f64, t5125: f64, t5147: f64, t56268: f64, t56271: f64, t56275: f64, t56277: f64, t1744: f64, t12464: f64, t16951: f64, t16955: f64, t16958: f64, t16962: f64, t16965: f64, t16966: f64, t3453: f64, t3471: f64, t45080: f64, t45085: f64, t45197: f64, t5143: f64, t56279: f64, t56281: f64, t56283: f64, t56286: f64, t56290: f64, t57799: f64, t12472: f64, t5142: f64, t3523: f64, t1187: f64, t12470: f64, t12481: f64, t12491: f64, t12497: f64, t12501: f64, t16979: f64, t16985: f64, t16989: f64, t17097: f64, t17151: f64, t17154: f64, t3496: f64, t3521: f64, t45061: f64, t45064: f64, t45157: f64, t45159: f64, t45168: f64, t5146: f64, t5163: f64, t5185: f64, t57802: f64, t57805: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57849, t57851, t57853, t57856, t57860) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3139(t1196, t12548, t5197, t16643, t3531, t16682, t1732, t3433, t12411, t12556, t1756, t43752);
        let (t57863, t57883) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3140(t16831, t300, t1198, t56176, t56183, t43830, t43832, t45000, t56151, t56155, t56159, t56163, t56167, t56174, t56181, t56185, t56187, t56189, t56194, t56198, t56203, t56207, t56209);
        let t57904 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3141(t56228, t43858, t43865, t43883, t43888, t43890, t43892, t43894, t43896, t56212, t56214, t56216, t56221, t56226, t56230, t56234, t56236, t56248, t56252, t56256);
        let (t57907, t57911, t57912) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3142(t422, t57883, t57904, t1189, t1196, t17150, t3495, t57820, t57822, t57825, t57827, t57829, t57831, t57833, t57835, t57837, t57840, t57842, t57846, t57849, t57851, t57853, t57856, t57860, t57863);
        let t57943 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3143(t1168, t12423, t12429, t12430, t12486, t12487, t12504, t12508, t12511, t16948, t16959, t17023, t17032, t17085, t17086, t1745, t1756, t1757, t3452, t3477, t3479, t45075, t45188, t45190, t45194, t5125, t5147, t56268, t56271, t56275, t56277);
        let t57967 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3144(t12429, t1744, t12423, t12430, t12464, t12508, t12511, t16951, t16955, t16958, t16962, t16965, t16966, t1745, t3452, t3453, t3471, t45080, t45085, t45197, t5143, t56279, t56281, t56283, t56286, t56290, t57799);
        let t58004 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3145(t12472, t5142, t17150, t3523, t1187, t12430, t12464, t12470, t12481, t12491, t12497, t12501, t16958, t16979, t16985, t16989, t17097, t17151, t17154, t1744, t3453, t3471, t3477, t3496, t3521, t45061, t45064, t45157, t45159, t45168, t5146, t5163, t5185, t57802, t57805);
    (t57849, t57851, t57853, t57856, t57860, t57863, t57907, t57911, t57912, t57943, t57967, t58004)
}
