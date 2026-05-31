//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 828/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk828<F: Float>(t2404: F, t6832: F, t2373: F, t2379: F, t2388: F, t2392: F, t2397: F, t2408: F, t3066: F, t3207: F, t6757: F, t6762: F, t6769: F, t6772: F, t6775: F, t6778: F, t6784: F, t6789: F, t6793: F, t6797: F, t6802: F, t6805: F, t6810: F, t6816: F, t6819: F, t6824: F, t6828: F, t827: F, t833: F) -> F {
    let t6833 = t6832 * t2404;
    let t6835 = t2408 * t6757 / F::cast_from(16.0_f64) - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t3207 * t6762 - t2388 * t2379 / F::cast_from(32.0_f64) - t2392 * t2379 / F::cast_from(32.0_f64) + t6769 * t6772 / F::cast_from(32.0_f64) + t6775 * t6778 / F::cast_from(32.0_f64) - t827 * t6784 / F::cast_from(16.0_f64) - t827 * t6789 / F::cast_from(16.0_f64) + t6793 * t6797 / F::cast_from(8.0_f64) + t6802 * t833 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t6805 - t2392 * t2373 / F::cast_from(16.0_f64) + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t3207 * t6810 + t2392 * t2397 / F::cast_from(32.0_f64) - t6816 * t6819 / F::cast_from(4.0_f64) + t3066 * t6824 / F::cast_from(16.0_f64) + t2408 * t6828 / F::cast_from(8.0_f64) - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t6833;
    t6835
}
