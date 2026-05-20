//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta400 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1483;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1484;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1485;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1486;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta400<F: Float>(t378: F, t42051: F, t11198: F, t340: F, t338: F, t3059: F, t11119: F, t384: F, t225: F, t3270: F, t41306: F, t41308: F, t41312: F, t41316: F, t41320: F, t41323: F, t41327: F, t41330: F, t41332: F, t41334: F, t41336: F, t41341: F, t41344: F, t41347: F, t41350: F, t41353: F, t41356: F, t41359: F, t41361: F, t41363: F, t41365: F, t41367: F, t41369: F, t3057: F, t3259: F, t1000: F, t1073: F, t1076: F, t1097: F, t11121: F, t11122: F, t11128: F, t11201: F, t11203: F, t11224: F, t11902: F, t12040: F, t12174: F, t12178: F, t3052: F, t3058: F, t3060: F, t3063: F, t3067: F, t386: F, t42001: F, t42033: F, t42038: F, t42041: F, t42044: F, t42047: F, t995: F, t996: F, t999: F, t367: F, t371: F, t373: F, t9291: F, t1058: F, t11907: F, t3197: F, t3201: F, t11962: F, t3231: F, t11973: F, t11904: F, t11773: F, t11865: F, t11941: F, t11942: F, t127: F, t11937: F, t11947: F, t3205: F, t3206: F, t676: F, t1063: F, t1066: F, t11286: F, t11663: F, t11687: F, t11774: F, t11776: F, t11859: F, t11994: F, t12024: F, t15609: F, t15758: F, t247: F, t3096: F, t3117: F, t366: F, t372: F, t375: F, t41310: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t42052, t42059, t42060, t42061, t42067, t42068, t42083) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1483::<F>(t378, t42051, t11198, t340, t338, t3059, t11119, t384, t225, t3270, t41306, t41308, t41312, t41316, t41320, t41323, t41327, t41330, t41332, t41334, t41336);
        let t42096 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1484::<F>(t41341, t41344, t41347, t41350, t41353, t41356, t41359, t41361, t41363, t41365, t41367, t41369);
        let (t42097, t42112) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1485::<F>(t42083, t42096, t3057, t3259, t1000, t1073, t1076, t1097, t11121, t11122, t11128, t11201, t11203, t11224, t11902, t12040, t12174, t12178, t3052, t3058, t3060, t3063, t3067, t386, t42001, t42033, t42038, t42041, t42044, t42047, t42052, t42060, t42061, t42067, t42068, t995, t996, t999);
        let (t42121, t42122, t42124, t42139, t42141, t42146, t42149) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1486::<F>(t367, t371, t373, t9291, t1058, t11907, t3197, t3201, t11962, t3231, t11973, t11904);
        let t42184 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1487::<F>(t11773, t11865, t11941, t11942, t127, t371, t11937, t11947, t3205, t3206, t676, t1063, t1066, t11286, t11663, t11687, t11774, t11776, t11859, t11994, t12024, t15609, t15758, t225, t247, t3096, t3117, t366, t372, t375, t41310, t42033, t42149);
    (t42059, t42061, t42097, t42112, t42121, t42122, t42124, t42139, t42141, t42146, t42184)
}
