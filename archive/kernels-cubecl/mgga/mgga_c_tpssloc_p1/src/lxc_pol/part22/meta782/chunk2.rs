//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2674/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2674<F: Float>(t1348: F, t1821: F, t19702: F, t19708: F, t19716: F, t19719: F, t19725: F, t20536: F, t225: F, t5272: F, t5280: F, t5283: F, t548: F, t550: F, t6404: F, t6408: F, t6411: F, t68: F, t74466: F, t74467: F, t74469: F, t74471: F, t74480: F, t74487: F, t74498: F, t74505: F, t74562: F) -> F {
    let t74564 = (-(t74466 + t74467 + t74469 + t74471 + t74480 + t74487 + t74498 + t74505) * t225 * t548 + F::cast_from(3.0_f64) * t20536 * t1348 + F::cast_from(9.0_f64) * t19702 * t1821 - F::cast_from(36.0_f64) * t6404 * t68 * t5280 + F::cast_from(9.0_f64) * t6404 * t5283 - F::cast_from(36.0_f64) * t5272 * t6408 + F::cast_from(180.0_f64) * t19708 * t19716 - F::cast_from(72.0_f64) * t19708 * t19719 + F::cast_from(9.0_f64) * t5272 * t6411 - F::cast_from(36.0_f64) * t19708 * t19725 + t74562) * t550;
    t74564
}
