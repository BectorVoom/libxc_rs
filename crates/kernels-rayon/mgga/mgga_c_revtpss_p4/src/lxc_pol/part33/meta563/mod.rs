//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1961;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1962;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta563(t30681: f64, t72: f64, t1927: f64, t7719: f64, t8143: f64, t2122: f64, t29532: f64, t1923: f64, t2123: f64, t26792: f64, t28154: f64, t29380: f64, t29388: f64, t29412: f64, t29513: f64, t29538: f64, t29544: f64, t29548: f64, t29551: f64, t29554: f64, t29562: f64, t7566: f64, t7702: f64, t7706: f64, t7709: f64, t8144: f64, t8147: f64, t5: f64, t30: f64, t265: f64, t393: f64, t117: f64, t2126: f64, t5883: f64, t29930: f64, t1469: f64, t2129: f64, t29726: f64, t45: f64, t5825: f64, t8161: f64, t2142: f64, t6587: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30682, t30683, t30686, t30689, t30714) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1961(t30681, t72, t1927, t7719, t8143, t2122, t29532, t1923, t2123, t26792, t28154, t29380, t29388, t29412, t29513, t29538, t29544, t29548, t29551, t29554, t29562, t7566, t7702, t7706, t7709, t8144, t8147);
        let (t30715, t30716, t30724, t30727, t30734, t30735) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1962(t5, t30, t265, t393, t30714, t117, t2126, t5883, t29930, t1469, t2129, t29726, t45, t5825, t8161, t2142, t6587, dens_threshold, rho0, zeta_threshold);
    (t30682, t30683, t30686, t30689, t30715, t30716, t30724, t30727, t30734, t30735)
}
