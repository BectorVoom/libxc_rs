//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 829/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk829<F: Float>(t13343: F, t17288: F, t13350: F, t4349: F, t605: F, t1382: F, t2497: F, t3599: F, t11298: F, t19933: F, t11556: F, t921: F) -> (F, F, F, F, F) {
    let t44678 = F::cast_from(6.0_f64) * t17288 * t13343;
    let t44684 = F::cast_from(6.0_f64) * t4349 * t13350 * t605;
    let t44687 = F::cast_from(2.0_f64) * t1382 * t3599 * t2497;
    let t44689 = F::cast_from(6.0_f64) * t19933 * t11298;
    let t44692 = F::cast_from(2.0_f64) * t1382 * t11556 * t921;
    (t44678, t44684, t44687, t44689, t44692)
}
