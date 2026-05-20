//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1890/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1890<F: Float>(t13272: F, t607: F, t10301: F, t1470: F, t2247: F, t4181: F, t4187: F, t94976: F, t1513: F, t94975: F, t28036: F, t94978: F) -> (F, F, F, F, F, F, F) {
    let t101230 = t13272 * t607;
    let t101237 = t10301 * t1470;
    let t101240 = t2247 * t4181;
    let t101243 = t2247 * t4187;
    let t101448 = F::new(22.0) / F::new(9.0) * t94976;
    let t101451 = t94975 * t1513;
    let t101453 = t94978 * t28036;
    (t101230, t101237, t101240, t101243, t101448, t101451, t101453)
}
