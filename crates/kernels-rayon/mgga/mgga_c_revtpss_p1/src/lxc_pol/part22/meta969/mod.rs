//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta969 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3234;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3235;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3236;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3237;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta969(t4401: f64, t606: f64, t61303: f64, t50865: f64, t50868: f64, t14325: f64, t18559: f64, t14369: f64, t4186: f64, t40156: f64, t11084: f64, t2403: f64, t5962: f64, t61292: f64, t61293: f64, t61295: f64, t61297: f64, t61300: f64, t61302: f64, t2439: f64, t6041: f64, t780: f64, t785: f64, t4533: f64, t18821: f64, t2471: f64, t18814: f64, t2435: f64, t14476: f64, t1580: f64, t689: f64, t18662: f64, t41070: f64, t686: f64, t72: f64, t18658: f64, t786: f64, t789: f64, t18796: f64, t2465: f64, t2470: f64, t15011: f64, t18800: f64, t2770: f64, t2772: f64, t39549: f64, t39550: f64, t4487: f64, t50155: f64, t50164: f64, t50166: f64, t50169: f64, t50174: f64, t50178: f64, t50183: f64, t865: f64, t18811: f64, t18825: f64, t2453: f64, t2458: f64, t6042: f64, t18785: f64, t779: f64, t18316: f64, t887: f64, t14979: f64, t15029: f64, t39554: f64, t39557: f64, t39558: f64, t39562: f64, t39565: f64, t39567: f64, t39573: f64, t4474: f64, t50161: f64, t50186: f64, t50198: f64, t50201: f64, t50205: f64, t50209: f64, t50240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61306, t61310, t61311, t61313, t61316, t61317, t61318) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3234(t4401, t606, t61303, t50865, t50868, t14325, t18559, t14369, t4186, t40156, t11084, t2403, t5962, t61292, t61293, t61295, t61297, t61300, t61302);
        let (t61324, t61326, t61330, t61337, t61344) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3235(t2439, t6041, t780, t785, t4533, t18821, t2471, t18814, t2435, t14476, t1580, t689);
        let t61358 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3236(t18662, t41070, t686, t72, t18658, t786, t789, t18796, t2465, t2470, t15011, t18800, t2770, t2772, t39549, t39550, t4487, t50155, t50164, t50166, t50169, t50174, t50178, t50183, t61324, t61326, t61330, t61337, t61344, t865);
        let t61387 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3237(t18811, t2435, t18825, t2453, t2458, t6042, t18785, t689, t779, t18316, t887, t14979, t15029, t39554, t39557, t39558, t39562, t39565, t39567, t39573, t4474, t50161, t50186, t50198, t50201, t50205, t50209, t50240);
    (t61306, t61310, t61311, t61313, t61316, t61317, t61318, t61358, t61387)
}
