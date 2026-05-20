//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta744 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2526;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2527;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta744<F: Float>(t51564: F, t10115: F, t1576: F, t14593: F, t2470: F, t874: F, t10538: F, t14605: F, t49180: F, t10535: F, t136: F, t2457: F, t4424: F, t14523: F, t9285: F, t10073: F, t14496: F, t14946: F, t2710: F, t14598: F, t14600: F, t2434: F, t836: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t51565, t51578, t51588, t51604, t51614) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2526::<F>(t51564, t10115, t1576, t14593, t2470, t874, t10538, t14605, t49180, t10535, t136, t2457, t4424);
        let (t51615, t51635, t51637, t51646, t51657) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2527::<F>(t51614, t10535, t14523, t9285, t10073, t14496, t14946, t2710, t14598, t14600, t2434, t836);
    (t51565, t51578, t51588, t51604, t51615, t51635, t51637, t51646, t51657)
}
