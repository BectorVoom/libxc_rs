//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2660/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2660<F: Float>(t28: F, t1081: F, t1302: F, t16003: F, t16006: F, t18196: F, t19618: F, t20385: F, t20390: F, t2219: F, t3711: F, t39877: F, t5178: F, t71090: F, t73995: F, t73998: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t74353 = piecewise3::<F>(t29, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t39877 * t20385 * t1081 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t19618 * t2219 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t16003 * t73995 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t16006 * t73998 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5178 * t18196 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3711 * t20390 * t1081 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1302 * t71090);
    t74353
}
