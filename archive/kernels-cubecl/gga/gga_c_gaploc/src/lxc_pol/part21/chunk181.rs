//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 181/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk181<F: Float>(t241: F, t629: F, t367: F, t46: F, t372: F, t374: F, t231: F, t242: F, t337: F, t359: F, t4: F, t55: F, t624: F, t79: F) -> (F, F, F, F, F) {
    let t630 = F::cast_from(1.0_f64) / t241;
    let t631 = t629 * t630;
    let t637 = t46 * t367;
    let t638 = t372 * t374;
    let t642 = t231 * (F::cast_from(0.53236443333333333332e-3_f64) * t4 * t79 * t242 + F::cast_from(1.0_f64) * t624 * t631 - t337 - t359 + F::cast_from(0.18311555036753159941e-3_f64) * t4 * t79 * t55 + F::cast_from(0.58482233974552040708e0_f64) * t637 * t638);
    (t630, t631, t637, t638, t642)
}
