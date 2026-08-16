//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1577;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1578;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta311(t10832: f64, t2484: f64, t2749: f64, t836: f64, t853: f64, t2662: f64, t2661: f64, t221: f64, t2485: f64, t2646: f64, t2482: f64, t596: f64, t823: f64, t2487: f64, t10794: f64, t10799: f64, t10803: f64, t10807: f64, t10812: f64, t10816: f64, t10820: f64, t10824: f64, t10826: f64, t10828: f64, t2745: f64, t4362: f64, t825: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10833, t10837, t10838, t10841, t10842, t10845) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1577(t10832, t2484, t2749, t836, t853, t2662, t2661, t221, t2485, t2646, t2482, t596, t823);
        let (t10846, t10848) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1578(t10845, t2487, t10794, t10799, t10803, t10807, t10812, t10816, t10820, t10824, t10826, t10828, t10833, t10838, t10842, t2745, t4362, t825, t851);
    (t10833, t10837, t10838, t10841, t10842, t10845, t10846, t10848)
}
