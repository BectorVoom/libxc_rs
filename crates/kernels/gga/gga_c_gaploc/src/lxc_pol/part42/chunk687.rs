//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 687/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk687<F: Float>(t13327: F, t2268: F, t12831: F, t11288: F, t921: F, t3366: F, t8045: F, t3553: F, t6556: F, t4349: F, t2355: F, t3599: F) -> (F, F, F, F, F, F, F, F) {
    let t13329 = F::new(0.28455006635676149599e-1) * t2268 * t13327;
    let t13330 = F::new(0.142275033178380748e-1) * t12831;
    let t13334 = t11288 * t921;
    let t13338 = F::new(4.0) * t8045 * t3366;
    let t13342 = F::new(2.0) * t6556 * t3553;
    let t13343 = t3553 * t921;
    let t13345 = F::new(6.0) * t4349 * t13343;
    let t13349 = t2355 * t3599;
    (t13329, t13330, t13334, t13338, t13342, t13343, t13345, t13349)
}
