//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 771/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk771<F: Float>(t2409: F, t3067: F, t6822: F, t2410: F, t6781: F, t329: F, t369: F, t838: F, t2404: F, t2373: F, t2379: F, t2388: F, t2392: F, t2397: F, t2408: F, t3066: F, t3207: F, t6757: F, t6762: F, t6769: F, t6772: F, t6775: F, t6778: F, t6784: F, t6789: F, t6793: F, t6797: F, t6802: F, t6805: F, t6810: F, t6816: F, t6819: F, t827: F, t833: F) -> (F, F, F, F) {
    let t6824 = t2409 * t3067 * t6822;
    let t6828 = t2409 * t6781 * t2410;
    let t6832 = t329 * t838 * t369;
    let t6833 = t6832 * t2404;
    let t6835 = t2408 * t6757 / 16.0 - 3.0 / 16.0 * t3207 * t6762 - t2388 * t2379 / 32.0 - t2392 * t2379 / 32.0 + t6769 * t6772 / 32.0 + t6775 * t6778 / 32.0 - t827 * t6784 / 16.0 - t827 * t6789 / 16.0 + t6793 * t6797 / 8.0 + t6802 * t833 / 96.0 + 7.0 / 24.0 * t6805 - t2392 * t2373 / 16.0 + 3.0 / 16.0 * t3207 * t6810 + t2392 * t2397 / 32.0 - t6816 * t6819 / 4.0 + t3066 * t6824 / 16.0 + t2408 * t6828 / 8.0 - 7.0 / 16.0 * t6833;
    (t6824, t6828, t6832, t6835)
}
