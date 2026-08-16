//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta663 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2160;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2161;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2162;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta663(t7933: f64, t9593: f64, t28196: f64, t28198: f64, t30138: f64, t7003: f64, t13426: f64, t7735: f64, t18227: f64, t27137: f64, t4248: f64, t30123: f64, t95088: f64, t1353: f64, t6922: f64, t25082: f64, t8717: f64, t30088: f64, t689: f64, t25904: f64, t25899: f64, t30105: f64, t94395: f64, t94649: f64, t30071: f64, t7308: f64, t94378: f64, t94388: f64, t94392: f64, t97682: f64, t97687: f64, t97690: f64, t97698: f64, t97702: f64, t97707: f64, t27989: f64, t98380: f64, t6919: f64, t7242: f64, t1904: f64, t2022: f64, t22386: f64, t25924: f64, t27868: f64, t27980: f64, t28008: f64, t6895: f64, t7274: f64, t7295: f64, t7296: f64, t75188: f64, t75267: f64, t7930: f64, t94409: f64, t94580: f64, t94591: f64, t94593: f64, t97719: f64, t97734: f64, t98056: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t108103, t108105, t108107, t108109, t108111, t108117) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2160(t7933, t9593, t28196, t28198, t30138, t7003, t13426, t7735, t18227, t27137, t4248, t30123, t95088);
        let (t108129, t108145) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2161(t1353, t6922, t25082, t8717, t30088, t689, t25904, t25899, t30105, t94395, t94649, t30071, t7308, t94378, t94388, t94392, t97682, t97687, t97690, t97698, t97702, t97707);
        let t108172 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2162(t27989, t98380, t689, t6919, t7242, t1904, t2022, t22386, t25924, t27868, t27980, t28008, t6895, t7274, t7295, t7296, t75188, t75267, t7930, t94409, t94580, t94591, t94593, t97719, t97734, t98056);
    (t108103, t108105, t108107, t108109, t108111, t108117, t108129, t108145, t108172)
}
