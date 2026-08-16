//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1104/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1104<F: Float>(t1936: F, t34359: F, t572: F, t28986: F, t7553: F, t7741: F, t196: F, t197: F, t8237: F, t13272: F, t8736: F, t8142: F, t8435: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34360 = t34359 * t1936;
    let t34362 = F::cast_from(6.0_f64) * t572 * t34360;
    let t34363 = t28986 * t1936;
    let t34365 = F::cast_from(6.0_f64) * t572 * t34363;
    let t34366 = t7553 * t7741;
    let t34368 = F::cast_from(6.0_f64) * t572 * t34366;
    let t34399 = t8237 * t196 * t197;
    let t34402 = t13272 * t8736;
    let t34409 = t8435 * t8142;
    (t34360, t34362, t34363, t34365, t34366, t34368, t34399, t34402, t34409)
}
