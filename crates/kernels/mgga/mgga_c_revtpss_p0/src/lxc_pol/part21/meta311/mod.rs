//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1577;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1578;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta311<F: Float>(t10832: F, t2484: F, t2749: F, t836: F, t853: F, t2662: F, t2661: F, t221: F, t2485: F, t2646: F, t2482: F, t596: F, t823: F, t2487: F, t10794: F, t10799: F, t10803: F, t10807: F, t10812: F, t10816: F, t10820: F, t10824: F, t10826: F, t10828: F, t2745: F, t4362: F, t825: F, t851: F) -> (F, F, F, F, F, F, F, F) {
        let (t10833, t10837, t10838, t10841, t10842, t10845) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1577::<F>(t10832, t2484, t2749, t836, t853, t2662, t2661, t221, t2485, t2646, t2482, t596, t823);
        let (t10846, t10848) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1578::<F>(t10845, t2487, t10794, t10799, t10803, t10807, t10812, t10816, t10820, t10824, t10826, t10828, t10833, t10838, t10842, t2745, t4362, t825, t851);
    (t10833, t10837, t10838, t10841, t10842, t10845, t10846, t10848)
}
