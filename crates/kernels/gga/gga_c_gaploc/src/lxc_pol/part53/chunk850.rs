//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 850/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk850<F: Float>(t12887: F, t1641: F, t39624: F, t39626: F, t39632: F, t39637: F, t39642: F, t39646: F, t39648: F, t39650: F, t471: F, t12782: F, t64: F) -> (F, F, F) {
    let t42099 = F::cast_from(0.92023022289409799224e1_f64) * t1641 * t12887;
    let t42111 = (F::new(21.0) / F::new(512.0) * t39624 + F::new(357.0) / F::new(16384.0) * t39626 - F::new(189.0) / F::new(262144.0) * t39632 + F::new(189.0) / F::new(0.16777216e8) * t39637 - F::new(63.0) / F::new(0.16777216e8) * t39642 + F::new(63.0) / F::new(262144.0) * t39646 - F::new(119.0) / F::new(16384.0) * t39648 - F::new(7.0) / F::new(512.0) * t39650) * t471;
    let t42113 = F::new(4.0) / F::new(3.0) * t12782 * t64;
    (t42099, t42111, t42113)
}
