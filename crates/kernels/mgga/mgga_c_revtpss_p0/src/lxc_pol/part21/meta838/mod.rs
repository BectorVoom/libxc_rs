//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta838 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3139;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3140;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3141;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3142;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3143;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3144;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3145;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta838<F: Float>(t1196: F, t12548: F, t5197: F, t16643: F, t3531: F, t16682: F, t1732: F, t3433: F, t12411: F, t12556: F, t1756: F, t43752: F, t16831: F, t300: F, t1198: F, t56176: F, t56183: F, t43830: F, t43832: F, t45000: F, t56151: F, t56155: F, t56159: F, t56163: F, t56167: F, t56174: F, t56181: F, t56185: F, t56187: F, t56189: F, t56194: F, t56198: F, t56203: F, t56207: F, t56209: F, t56228: F, t43858: F, t43865: F, t43883: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t56212: F, t56214: F, t56216: F, t56221: F, t56226: F, t56230: F, t56234: F, t56236: F, t56248: F, t56252: F, t56256: F, t422: F, t1189: F, t17150: F, t3495: F, t57820: F, t57822: F, t57825: F, t57827: F, t57829: F, t57831: F, t57833: F, t57835: F, t57837: F, t57840: F, t57842: F, t57846: F, t1168: F, t12423: F, t12429: F, t12430: F, t12486: F, t12487: F, t12504: F, t12508: F, t12511: F, t16948: F, t16959: F, t17023: F, t17032: F, t17085: F, t17086: F, t1745: F, t1757: F, t3452: F, t3477: F, t3479: F, t45075: F, t45188: F, t45190: F, t45194: F, t5125: F, t5147: F, t56268: F, t56271: F, t56275: F, t56277: F, t1744: F, t12464: F, t16951: F, t16955: F, t16958: F, t16962: F, t16965: F, t16966: F, t3453: F, t3471: F, t45080: F, t45085: F, t45197: F, t5143: F, t56279: F, t56281: F, t56283: F, t56286: F, t56290: F, t57799: F, t12472: F, t5142: F, t3523: F, t1187: F, t12470: F, t12481: F, t12491: F, t12497: F, t12501: F, t16979: F, t16985: F, t16989: F, t17097: F, t17151: F, t17154: F, t3496: F, t3521: F, t45061: F, t45064: F, t45157: F, t45159: F, t45168: F, t5146: F, t5163: F, t5185: F, t57802: F, t57805: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t57849, t57851, t57853, t57856, t57860) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3139::<F>(t1196, t12548, t5197, t16643, t3531, t16682, t1732, t3433, t12411, t12556, t1756, t43752);
        let (t57863, t57883) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3140::<F>(t16831, t300, t1198, t56176, t56183, t43830, t43832, t45000, t56151, t56155, t56159, t56163, t56167, t56174, t56181, t56185, t56187, t56189, t56194, t56198, t56203, t56207, t56209);
        let t57904 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3141::<F>(t56228, t43858, t43865, t43883, t43888, t43890, t43892, t43894, t43896, t56212, t56214, t56216, t56221, t56226, t56230, t56234, t56236, t56248, t56252, t56256);
        let (t57907, t57911, t57912) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3142::<F>(t422, t57883, t57904, t1189, t1196, t17150, t3495, t57820, t57822, t57825, t57827, t57829, t57831, t57833, t57835, t57837, t57840, t57842, t57846, t57849, t57851, t57853, t57856, t57860, t57863);
        let t57943 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3143::<F>(t1168, t12423, t12429, t12430, t12486, t12487, t12504, t12508, t12511, t16948, t16959, t17023, t17032, t17085, t17086, t1745, t1756, t1757, t3452, t3477, t3479, t45075, t45188, t45190, t45194, t5125, t5147, t56268, t56271, t56275, t56277);
        let t57967 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3144::<F>(t12429, t1744, t12423, t12430, t12464, t12508, t12511, t16951, t16955, t16958, t16962, t16965, t16966, t1745, t3452, t3453, t3471, t45080, t45085, t45197, t5143, t56279, t56281, t56283, t56286, t56290, t57799);
        let t58004 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3145::<F>(t12472, t5142, t17150, t3523, t1187, t12430, t12464, t12470, t12481, t12491, t12497, t12501, t16958, t16979, t16985, t16989, t17097, t17151, t17154, t1744, t3453, t3471, t3477, t3496, t3521, t45061, t45064, t45157, t45159, t45168, t5146, t5163, t5185, t57802, t57805);
    (t57849, t57851, t57853, t57856, t57860, t57863, t57907, t57911, t57912, t57943, t57967, t58004)
}
