//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1326;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1327;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta325<F: Float>(t10705: F, t2674: F, t231: F, t243: F, t2645: F, t2662: F, t2661: F, t2652: F, t2656: F, t2482: F, t596: F, t849: F, t2677: F, t2665: F, t9775: F, t2681: F, t820: F, t857: F, t240: F, t2719: F, t2722: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10706, t10709, t10711, t10713, t10716) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1326::<F>(t10705, t2674, t231, t243, t2645, t2662, t2661, t2652, t2656, t2482, t596, t849);
        let (t10717, t10719, t10722, t10723, t10726, t10727) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1327::<F>(t10716, t2677, t2665, t9775, t2681, t820, t849, t857, t240, t2719, t243, t2722);
    (t10706, t10709, t10711, t10713, t10716, t10717, t10719, t10722, t10723, t10726, t10727)
}
