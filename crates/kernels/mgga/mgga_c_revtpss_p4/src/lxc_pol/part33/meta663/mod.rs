//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta663 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2160;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2161;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2162;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta663<F: Float>(t7933: F, t9593: F, t28196: F, t28198: F, t30138: F, t7003: F, t13426: F, t7735: F, t18227: F, t27137: F, t4248: F, t30123: F, t95088: F, t1353: F, t6922: F, t25082: F, t8717: F, t30088: F, t689: F, t25904: F, t25899: F, t30105: F, t94395: F, t94649: F, t30071: F, t7308: F, t94378: F, t94388: F, t94392: F, t97682: F, t97687: F, t97690: F, t97698: F, t97702: F, t97707: F, t27989: F, t98380: F, t6919: F, t7242: F, t1904: F, t2022: F, t22386: F, t25924: F, t27868: F, t27980: F, t28008: F, t6895: F, t7274: F, t7295: F, t7296: F, t75188: F, t75267: F, t7930: F, t94409: F, t94580: F, t94591: F, t94593: F, t97719: F, t97734: F, t98056: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t108103, t108105, t108107, t108109, t108111, t108117) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2160::<F>(t7933, t9593, t28196, t28198, t30138, t7003, t13426, t7735, t18227, t27137, t4248, t30123, t95088);
        let (t108129, t108145) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2161::<F>(t1353, t6922, t25082, t8717, t30088, t689, t25904, t25899, t30105, t94395, t94649, t30071, t7308, t94378, t94388, t94392, t97682, t97687, t97690, t97698, t97702, t97707);
        let t108172 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2162::<F>(t27989, t98380, t689, t6919, t7242, t1904, t2022, t22386, t25924, t27868, t27980, t28008, t6895, t7274, t7295, t7296, t75188, t75267, t7930, t94409, t94580, t94591, t94593, t97719, t97734, t98056);
    (t108103, t108105, t108107, t108109, t108111, t108117, t108129, t108145, t108172)
}
