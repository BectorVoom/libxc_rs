//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta969 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3234;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3235;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3236;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3237;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta969<F: Float>(t4401: F, t606: F, t61303: F, t50865: F, t50868: F, t14325: F, t18559: F, t14369: F, t4186: F, t40156: F, t11084: F, t2403: F, t5962: F, t61292: F, t61293: F, t61295: F, t61297: F, t61300: F, t61302: F, t2439: F, t6041: F, t780: F, t785: F, t4533: F, t18821: F, t2471: F, t18814: F, t2435: F, t14476: F, t1580: F, t689: F, t18662: F, t41070: F, t686: F, t72: F, t18658: F, t786: F, t789: F, t18796: F, t2465: F, t2470: F, t15011: F, t18800: F, t2770: F, t2772: F, t39549: F, t39550: F, t4487: F, t50155: F, t50164: F, t50166: F, t50169: F, t50174: F, t50178: F, t50183: F, t865: F, t18811: F, t18825: F, t2453: F, t2458: F, t6042: F, t18785: F, t779: F, t18316: F, t887: F, t14979: F, t15029: F, t39554: F, t39557: F, t39558: F, t39562: F, t39565: F, t39567: F, t39573: F, t4474: F, t50161: F, t50186: F, t50198: F, t50201: F, t50205: F, t50209: F, t50240: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t61306, t61310, t61311, t61313, t61316, t61317, t61318) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3234::<F>(t4401, t606, t61303, t50865, t50868, t14325, t18559, t14369, t4186, t40156, t11084, t2403, t5962, t61292, t61293, t61295, t61297, t61300, t61302);
        let (t61324, t61326, t61330, t61337, t61344) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3235::<F>(t2439, t6041, t780, t785, t4533, t18821, t2471, t18814, t2435, t14476, t1580, t689);
        let t61358 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3236::<F>(t18662, t41070, t686, t72, t18658, t786, t789, t18796, t2465, t2470, t15011, t18800, t2770, t2772, t39549, t39550, t4487, t50155, t50164, t50166, t50169, t50174, t50178, t50183, t61324, t61326, t61330, t61337, t61344, t865);
        let t61387 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3237::<F>(t18811, t2435, t18825, t2453, t2458, t6042, t18785, t689, t779, t18316, t887, t14979, t15029, t39554, t39557, t39558, t39562, t39565, t39567, t39573, t4474, t50161, t50186, t50198, t50201, t50205, t50209, t50240);
    (t61306, t61310, t61311, t61313, t61316, t61317, t61318, t61358, t61387)
}
