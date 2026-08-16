//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1600/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1600(t1470: f64, t1471: f64, t1486: f64, t1494: f64, t1927: f64, t21686: f64, t22671: f64, t22672: f64, t22673: f64, t22676: f64, t22681: f64, t22718: f64, t22739: f64, t36: f64, t5826: f64, t5827: f64, t5830: f64, t5854: f64, t5869: f64, t70: f64, t85: f64, t87126: f64) -> f64 {
    let t87221 = -t21686 * t1927 * t22671 / 3.0_f64 - t36 * t87126 * t70 * t85 / 12.0_f64 - t22672 * t1486 * t85 / 3.0_f64 - t22673 * t1494 / 3.0_f64 - t5826 * t5854 * t85 / 2.0_f64 - t22676 * t1494 - t5827 * t5869 / 2.0_f64 - t1470 * t22718 * t85 / 3.0_f64 - t22681 * t1494 - t5830 * t5869 - t1471 * t22739 / 3.0_f64;
    t87221
}
