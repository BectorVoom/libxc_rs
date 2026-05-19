//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 202/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk202<F: Float>(t241: F, t711: F, t374: F, t46: F, t379: F, t381: F, t231: F, t242: F, t344: F, t366: F, t4: F, t55: F, t706: F, t79: F) -> (F, F, F, F, F) {
    let t712 = F::new(1.0) / t241;
    let t713 = t711 * t712;
    let t719 = t46 * t374;
    let t720 = t379 * t381;
    let t724 = t231 * (F::cast_from(0.53236443333333333332e-3_f64) * t4 * t79 * t242 + F::new(1.0) * t706 * t713 - t344 - t366 + F::cast_from(0.18311555036753159941e-3_f64) * t4 * t79 * t55 + F::cast_from(0.58482233974552040708e0_f64) * t719 * t720);
    (t712, t713, t719, t720, t724)
}
