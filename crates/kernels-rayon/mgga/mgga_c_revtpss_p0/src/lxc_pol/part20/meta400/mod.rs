//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta400 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1483;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1484;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1485;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1486;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta400(t378: f64, t42051: f64, t11198: f64, t340: f64, t338: f64, t3059: f64, t11119: f64, t384: f64, t225: f64, t3270: f64, t41306: f64, t41308: f64, t41312: f64, t41316: f64, t41320: f64, t41323: f64, t41327: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41341: f64, t41344: f64, t41347: f64, t41350: f64, t41353: f64, t41356: f64, t41359: f64, t41361: f64, t41363: f64, t41365: f64, t41367: f64, t41369: f64, t3057: f64, t3259: f64, t1000: f64, t1073: f64, t1076: f64, t1097: f64, t11121: f64, t11122: f64, t11128: f64, t11201: f64, t11203: f64, t11224: f64, t11902: f64, t12040: f64, t12174: f64, t12178: f64, t3052: f64, t3058: f64, t3060: f64, t3063: f64, t3067: f64, t386: f64, t42001: f64, t42033: f64, t42038: f64, t42041: f64, t42044: f64, t42047: f64, t995: f64, t996: f64, t999: f64, t367: f64, t371: f64, t373: f64, t9291: f64, t1058: f64, t11907: f64, t3197: f64, t3201: f64, t11962: f64, t3231: f64, t11973: f64, t11904: f64, t11773: f64, t11865: f64, t11941: f64, t11942: f64, t127: f64, t11937: f64, t11947: f64, t3205: f64, t3206: f64, t676: f64, t1063: f64, t1066: f64, t11286: f64, t11663: f64, t11687: f64, t11774: f64, t11776: f64, t11859: f64, t11994: f64, t12024: f64, t15609: f64, t15758: f64, t247: f64, t3096: f64, t3117: f64, t366: f64, t372: f64, t375: f64, t41310: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42052, t42059, t42060, t42061, t42067, t42068, t42083) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1483(t378, t42051, t11198, t340, t338, t3059, t11119, t384, t225, t3270, t41306, t41308, t41312, t41316, t41320, t41323, t41327, t41330, t41332, t41334, t41336);
        let t42096 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1484(t41341, t41344, t41347, t41350, t41353, t41356, t41359, t41361, t41363, t41365, t41367, t41369);
        let (t42097, t42112) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1485(t42083, t42096, t3057, t3259, t1000, t1073, t1076, t1097, t11121, t11122, t11128, t11201, t11203, t11224, t11902, t12040, t12174, t12178, t3052, t3058, t3060, t3063, t3067, t386, t42001, t42033, t42038, t42041, t42044, t42047, t42052, t42060, t42061, t42067, t42068, t995, t996, t999);
        let (t42121, t42122, t42124, t42139, t42141, t42146, t42149) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1486(t367, t371, t373, t9291, t1058, t11907, t3197, t3201, t11962, t3231, t11973, t11904);
        let t42184 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1487(t11773, t11865, t11941, t11942, t127, t371, t11937, t11947, t3205, t3206, t676, t1063, t1066, t11286, t11663, t11687, t11774, t11776, t11859, t11994, t12024, t15609, t15758, t225, t247, t3096, t3117, t366, t372, t375, t41310, t42033, t42149);
    (t42059, t42061, t42097, t42112, t42121, t42122, t42124, t42139, t42141, t42146, t42184)
}
