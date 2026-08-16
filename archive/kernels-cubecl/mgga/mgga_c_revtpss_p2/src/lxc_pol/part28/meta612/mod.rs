//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2137;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2138;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2139;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2140;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta612<F: Float>(t28184: F, t7235: F, t2014: F, t25190: F, t28176: F, t1907: F, t4135: F, t28196: F, t28197: F, t28173: F, t25188: F, t7901: F, t28189: F, t7900: F, t94358: F, t10416: F, t13435: F, t7746: F, t98522: F, t98525: F, t98528: F, t98530: F, t98532: F, t98534: F, t98537: F, t98539: F, t98541: F, t98544: F, t13716: F, t1450: F, t7237: F, t18163: F, t7735: F, t27137: F, t4254: F, t25082: F, t75353: F, t8717: F, t7311: F, t9593: F, t28198: F, t28166: F, t7234: F, t28168: F, t27153: F, t32113: F, t8995: F, t28199: F, t28021: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t98546, t98549, t98553, t98555, t98557) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2137::<F>(t28184, t7235, t2014, t25190, t28176, t1907, t4135, t28196, t28197, t28173, t25188, t7901);
        let t98563 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2138::<F>(t28189, t7235, t2014, t7900, t94358, t10416, t13435, t7746, t98522, t98525, t98528, t98530, t98532, t98534, t98537, t98539, t98541, t98544, t98546, t98549, t98553, t98555, t98557);
        let (t98567, t98569, t98571, t98574, t98575) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2139::<F>(t13716, t1450, t2014, t7237, t18163, t7735, t27137, t4254, t25082, t75353, t8717, t7311, t9593);
        let (t98578, t98581, t98584, t98590, t98594) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2140::<F>(t28196, t28198, t98575, t28166, t7234, t28168, t25082, t27153, t32113, t8995, t28199, t28021, t7235);
    (t98563, t98567, t98569, t98571, t98574, t98578, t98581, t98584, t98590, t98594)
}
