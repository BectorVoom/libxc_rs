//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1032/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1032<F: Float>(t20757: F, t20305: F, t20720: F, t20725: F, t20731: F, t20733: F, t20734: F, t20739: F, t20746: F, t20750: F, t20753: F, t20755: F, t2258: F, t2345: F, t3247: F, t6275: F, t6276: F, t6287: F, t904: F) -> (F, F) {
    let t20758 = 7.0 / 72.0 * t20757;
    let t20759 = -3.0 / 32.0 * t3247 * t2345 * t20305 * t6287 + 7.0 / 576.0 * t20720 - t20725 + t20731 - 5.0 / 16.0 * t20733 * t904 * t20734 * t2258 + t6275 * t6276 * t20739 / 16.0 + t20746 + t20750 + t20753 + t20755 + t20758;
    (t20758, t20759)
}
