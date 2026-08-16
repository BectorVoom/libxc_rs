//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1961;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1962;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta563<F: Float>(t30681: F, t72: F, t1927: F, t7719: F, t8143: F, t2122: F, t29532: F, t1923: F, t2123: F, t26792: F, t28154: F, t29380: F, t29388: F, t29412: F, t29513: F, t29538: F, t29544: F, t29548: F, t29551: F, t29554: F, t29562: F, t7566: F, t7702: F, t7706: F, t7709: F, t8144: F, t8147: F, t5: F, t30: F, t265: F, t393: F, t117: F, t2126: F, t5883: F, t29930: F, t1469: F, t2129: F, t29726: F, t45: F, t5825: F, t8161: F, t2142: F, t6587: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t30682, t30683, t30686, t30689, t30714) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1961::<F>(t30681, t72, t1927, t7719, t8143, t2122, t29532, t1923, t2123, t26792, t28154, t29380, t29388, t29412, t29513, t29538, t29544, t29548, t29551, t29554, t29562, t7566, t7702, t7706, t7709, t8144, t8147);
        let (t30715, t30716, t30724, t30727, t30734, t30735) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1962::<F>(t5, t30, t265, t393, t30714, t117, t2126, t5883, t29930, t1469, t2129, t29726, t45, t5825, t8161, t2142, t6587, dens_threshold, rho0, zeta_threshold);
    (t30682, t30683, t30686, t30689, t30715, t30716, t30724, t30727, t30734, t30735)
}
