//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta231 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1025;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1026;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1027;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta231<F: Float>(t10811: F, t2751: F, t2681: F, t820: F, t823: F, t839: F, t2430: F, t775: F, t2477: F, t828: F, t222: F, t9727: F, t2737: F, t9802: F, t10639: F, t827: F, t221: F, t2485: F, t2754: F, t2484: F, t2749: F, t836: F, t853: F, t2662: F, t2661: F, t2646: F, t2482: F, t596: F, t2487: F, t10794: F, t10799: F, t10803: F, t10807: F, t2745: F, t4362: F, t825: F, t851: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10812, t10815, t10816, t10818, t10820, t10824) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1025::<F>(t10811, t2751, t2681, t820, t823, t839, t2430, t775, t2477, t828, t222, t9727);
        let (t10826, t10828, t10832, t10833, t10836) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1026::<F>(t2737, t9802, t10639, t827, t828, t221, t2485, t2754, t2484, t2749, t836, t853);
        let (t10837, t10841, t10845, t10848) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1027::<F>(t10836, t2662, t2661, t221, t2485, t2646, t2484, t2482, t596, t823, t2487, t10794, t10799, t10803, t10807, t10812, t10816, t10820, t10824, t10826, t10828, t10833, t2745, t4362, t825, t851);
    (t10815, t10818, t10820, t10828, t10832, t10837, t10841, t10845, t10848)
}
