//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1776;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1777;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta326(t10815: f64, t839: f64, t222: f64, t9727: f64, t2737: f64, t9802: f64, t221: f64, t2485: f64, t2754: f64, t2484: f64, t2749: f64, t836: f64, t853: f64, t2662: f64, t2661: f64, t2646: f64, t2482: f64, t596: f64, t823: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10816, t10824, t10826, t10832, t10833, t10836) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1776(t10815, t839, t222, t9727, t2737, t9802, t221, t2485, t2754, t2484, t2749, t836, t853);
        let (t10837, t10838, t10841, t10842, t10845) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1777(t10836, t2662, t2661, t221, t2485, t2646, t2484, t2482, t596, t823);
    (t10816, t10824, t10826, t10832, t10833, t10837, t10838, t10841, t10842, t10845)
}
