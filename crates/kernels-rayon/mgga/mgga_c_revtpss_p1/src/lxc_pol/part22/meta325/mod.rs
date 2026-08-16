//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1773;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1774;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1775;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta325(t136: f64, t853: f64, t220: f64, t124: f64, t836: f64, t2749: f64, t10777: f64, t2723: f64, t775: f64, t820: f64, t823: f64, t844: f64, t2751: f64, t2681: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10778, t10779) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1773(t136, t853, t220);
        let (t10782, t10783, t10786, t10811) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1774(t124, t836, t10779, t2749, t10777, t2723, t775, t820, t823, t844);
        let (t10812, t10815) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1775(t10811, t2751, t2681, t820, t823);
    (t10778, t10779, t10782, t10783, t10786, t10811, t10812, t10815)
}
