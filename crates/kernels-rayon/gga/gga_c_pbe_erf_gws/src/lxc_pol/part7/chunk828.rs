//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 828/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk828(t2404: f64, t6832: f64, t2373: f64, t2379: f64, t2388: f64, t2392: f64, t2397: f64, t2408: f64, t3066: f64, t3207: f64, t6757: f64, t6762: f64, t6769: f64, t6772: f64, t6775: f64, t6778: f64, t6784: f64, t6789: f64, t6793: f64, t6797: f64, t6802: f64, t6805: f64, t6810: f64, t6816: f64, t6819: f64, t6824: f64, t6828: f64, t827: f64, t833: f64) -> f64 {
    let t6833 = t6832 * t2404;
    let t6835 = t2408 * t6757 / 16.0_f64 - 3.0_f64 / 16.0_f64 * t3207 * t6762 - t2388 * t2379 / 32.0_f64 - t2392 * t2379 / 32.0_f64 + t6769 * t6772 / 32.0_f64 + t6775 * t6778 / 32.0_f64 - t827 * t6784 / 16.0_f64 - t827 * t6789 / 16.0_f64 + t6793 * t6797 / 8.0_f64 + t6802 * t833 / 96.0_f64 + 7.0_f64 / 24.0_f64 * t6805 - t2392 * t2373 / 16.0_f64 + 3.0_f64 / 16.0_f64 * t3207 * t6810 + t2392 * t2397 / 32.0_f64 - t6816 * t6819 / 4.0_f64 + t3066 * t6824 / 16.0_f64 + t2408 * t6828 / 8.0_f64 - 7.0_f64 / 16.0_f64 * t6833;
    t6835
}
