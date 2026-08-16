//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2137;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2138;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2139;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2140;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta612(t28184: f64, t7235: f64, t2014: f64, t25190: f64, t28176: f64, t1907: f64, t4135: f64, t28196: f64, t28197: f64, t28173: f64, t25188: f64, t7901: f64, t28189: f64, t7900: f64, t94358: f64, t10416: f64, t13435: f64, t7746: f64, t98522: f64, t98525: f64, t98528: f64, t98530: f64, t98532: f64, t98534: f64, t98537: f64, t98539: f64, t98541: f64, t98544: f64, t13716: f64, t1450: f64, t7237: f64, t18163: f64, t7735: f64, t27137: f64, t4254: f64, t25082: f64, t75353: f64, t8717: f64, t7311: f64, t9593: f64, t28198: f64, t28166: f64, t7234: f64, t28168: f64, t27153: f64, t32113: f64, t8995: f64, t28199: f64, t28021: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98546, t98549, t98553, t98555, t98557) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2137(t28184, t7235, t2014, t25190, t28176, t1907, t4135, t28196, t28197, t28173, t25188, t7901);
        let t98563 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2138(t28189, t7235, t2014, t7900, t94358, t10416, t13435, t7746, t98522, t98525, t98528, t98530, t98532, t98534, t98537, t98539, t98541, t98544, t98546, t98549, t98553, t98555, t98557);
        let (t98567, t98569, t98571, t98574, t98575) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2139(t13716, t1450, t2014, t7237, t18163, t7735, t27137, t4254, t25082, t75353, t8717, t7311, t9593);
        let (t98578, t98581, t98584, t98590, t98594) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2140(t28196, t28198, t98575, t28166, t7234, t28168, t25082, t27153, t32113, t8995, t28199, t28021, t7235);
    (t98563, t98567, t98569, t98571, t98574, t98578, t98581, t98584, t98590, t98594)
}
