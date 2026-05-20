//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1776;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1777;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta326<F: Float>(t10815: F, t839: F, t222: F, t9727: F, t2737: F, t9802: F, t221: F, t2485: F, t2754: F, t2484: F, t2749: F, t836: F, t853: F, t2662: F, t2661: F, t2646: F, t2482: F, t596: F, t823: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10816, t10824, t10826, t10832, t10833, t10836) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1776::<F>(t10815, t839, t222, t9727, t2737, t9802, t221, t2485, t2754, t2484, t2749, t836, t853);
        let (t10837, t10838, t10841, t10842, t10845) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1777::<F>(t10836, t2662, t2661, t221, t2485, t2646, t2484, t2482, t596, t823);
    (t10816, t10824, t10826, t10832, t10833, t10837, t10838, t10841, t10842, t10845)
}
