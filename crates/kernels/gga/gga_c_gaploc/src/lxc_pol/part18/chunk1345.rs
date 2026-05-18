//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1345/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1345<F: Float>(t16710: F, t1961: F, t3459: F, t1383: F, t3418: F, t4349: F, t10298: F, t4342: F, t7324: F, t9034: F, t6571: F, t8045: F) -> (F, F, F, F, F) {
    let t34003 = F::new(24.0) * t16710 * t3459 * t1961;
    let t34006 = F::new(6.0) * t4349 * t3418 * t1383;
    let t34008 = F::new(4.0) * t4342 * t10298;
    let t34010 = F::new(2.0) * t7324 * t9034;
    let t34012 = F::new(2.0) * t8045 * t6571;
    (t34003, t34006, t34008, t34010, t34012)
}
