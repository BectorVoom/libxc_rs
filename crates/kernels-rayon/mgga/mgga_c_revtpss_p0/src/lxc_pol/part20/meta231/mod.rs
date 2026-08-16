//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta231 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1025;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1026;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1027;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta231(t10811: f64, t2751: f64, t2681: f64, t820: f64, t823: f64, t839: f64, t2430: f64, t775: f64, t2477: f64, t828: f64, t222: f64, t9727: f64, t2737: f64, t9802: f64, t10639: f64, t827: f64, t221: f64, t2485: f64, t2754: f64, t2484: f64, t2749: f64, t836: f64, t853: f64, t2662: f64, t2661: f64, t2646: f64, t2482: f64, t596: f64, t2487: f64, t10794: f64, t10799: f64, t10803: f64, t10807: f64, t2745: f64, t4362: f64, t825: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10812, t10815, t10816, t10818, t10820, t10824) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1025(t10811, t2751, t2681, t820, t823, t839, t2430, t775, t2477, t828, t222, t9727);
        let (t10826, t10828, t10832, t10833, t10836) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1026(t2737, t9802, t10639, t827, t828, t221, t2485, t2754, t2484, t2749, t836, t853);
        let (t10837, t10841, t10845, t10848) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1027(t10836, t2662, t2661, t221, t2485, t2646, t2484, t2482, t596, t823, t2487, t10794, t10799, t10803, t10807, t10812, t10816, t10820, t10824, t10826, t10828, t10833, t2745, t4362, t825, t851);
    (t10815, t10818, t10820, t10828, t10832, t10837, t10841, t10845, t10848)
}
