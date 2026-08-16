//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1223;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1224;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta318(t10705: f64, t2674: f64, t231: f64, t243: f64, t2645: f64, t2662: f64, t2661: f64, t2652: f64, t2656: f64, t2482: f64, t596: f64, t849: f64, t2677: f64, t2665: f64, t9775: f64, t2681: f64, t820: f64, t857: f64, t240: f64, t2719: f64, t2722: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10706, t10709, t10711, t10713, t10716) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1223(t10705, t2674, t231, t243, t2645, t2662, t2661, t2652, t2656, t2482, t596, t849);
        let (t10717, t10719, t10722, t10723, t10726, t10727) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1224(t10716, t2677, t2665, t9775, t2681, t820, t849, t857, t240, t2719, t243, t2722);
    (t10706, t10709, t10711, t10713, t10716, t10717, t10719, t10722, t10723, t10726, t10727)
}
