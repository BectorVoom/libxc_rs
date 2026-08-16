//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2124;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2125;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta617<F: Float>(t28189: F, t7235: F, t2014: F, t7900: F, t94358: F, t13716: F, t1450: F, t7237: F, t18163: F, t7735: F, t27137: F, t4254: F, t25082: F, t75353: F, t8717: F, t7311: F, t9593: F, t28196: F, t28198: F, t28166: F, t7234: F, t28168: F, t27153: F, t32113: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t98559, t98562, t98567, t98569, t98571) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2124::<F>(t28189, t7235, t2014, t7900, t94358, t13716, t1450, t7237, t18163, t7735, t27137, t4254);
        let (t98574, t98578, t98581, t98584) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2125::<F>(t25082, t75353, t8717, t7311, t9593, t28196, t28198, t28166, t7234, t28168, t27153, t32113);
    (t98559, t98562, t98567, t98569, t98571, t98574, t98578, t98581, t98584)
}
