//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1274/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1274<F: Float>(t22331: F, t22351: F, t833: F, t852: F, t1306: F, t22162: F, t22164: F, t22167: F, t22169: F, t22171: F, t22175: F, t22184: F, t22188: F, t22313: F, t2461: F, t3282: F, t6362: F) -> (F, F) {
    let t22355 = F::new(1.0) * t833 * (t22331 + t22351) * t852;
    let t22356 = F::new(6.0) * t1306 * t2461 * t3282 * t6362 + t22162 + t22164 + t22167 - t22169 + t22171 + t22175 - t22184 - t22188 - t22313 + t22355;
    (t22355, t22356)
}
