//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta744 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2526;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2527;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta744(t51564: f64, t10115: f64, t1576: f64, t14593: f64, t2470: f64, t874: f64, t10538: f64, t14605: f64, t49180: f64, t10535: f64, t136: f64, t2457: f64, t4424: f64, t14523: f64, t9285: f64, t10073: f64, t14496: f64, t14946: f64, t2710: f64, t14598: f64, t14600: f64, t2434: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51565, t51578, t51588, t51604, t51614) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2526(t51564, t10115, t1576, t14593, t2470, t874, t10538, t14605, t49180, t10535, t136, t2457, t4424);
        let (t51615, t51635, t51637, t51646, t51657) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2527(t51614, t10535, t14523, t9285, t10073, t14496, t14946, t2710, t14598, t14600, t2434, t836);
    (t51565, t51578, t51588, t51604, t51615, t51635, t51637, t51646, t51657)
}
