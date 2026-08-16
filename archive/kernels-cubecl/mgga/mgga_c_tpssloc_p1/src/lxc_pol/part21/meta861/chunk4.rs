//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3126/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3126<F: Float>(t15376: F, t15420: F, t15419: F, t18211: F, t3447: F, t11575: F, t11579: F, t11584: F, t15268: F, t15321: F, t18409: F, t18416: F, t18420: F, t4908: F, t51975: F, t52013: F, t63298: F, t63302: F) -> F {
    let t64667 = t15376 * t15420;
    let t64686 = t3447 * t15419 * t18211;
    let t64694 = -F::cast_from(0.2962962962962962963e-2_f64) * t15376 * t15321 + F::cast_from(0.27777777777777777777e-3_f64) * t3447 * t11575 * t18409 - F::cast_from(0.13168724279835390946e-2_f64) * t64667 - F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t4908 * t63298 - F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t4908 * t63302 + F::cast_from(0.27777777777777777777e-3_f64) * t3447 * t18416 * t11579 + F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t18416 * t11584 - F::cast_from(0.33333333333333333332e-2_f64) * t3447 * t51975 * t15268 - F::cast_from(0.55555555555555555554e-3_f64) * t52013 + F::cast_from(0.14814814814814814814e-2_f64) * t64686 + F::cast_from(0.27777777777777777777e-3_f64) * t3447 * t18420 * t11579 + F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t18420 * t11584;
    t64694
}
