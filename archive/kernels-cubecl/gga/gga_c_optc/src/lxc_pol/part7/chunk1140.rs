//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1140/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1140<F: Float>(t2296: F, t2301: F, t2302: F, t2315: F, t23578: F, t23618: F, t23649: F, t23687: F, t23691: F, t23694: F, t23699: F, t23708: F, t23709: F, t23715: F, t23732: F, t23745: F, t23758: F, t23771: F, t350: F, t8335: F, t8338: F, t8345: F, t8346: F, t8349: F, t8376: F, t974: F, t979: F) -> F {
    let t23775 = (t23578 + t23618 + t23649 + t23687) * t350 - F::cast_from(4.0_f64) * t23691 * t979 + F::cast_from(12.0_f64) * t23694 * t2302 - F::cast_from(6.0_f64) * t8335 * t2315 - F::cast_from(24.0_f64) * t23699 * t8346 + F::cast_from(24.0_f64) * t8338 * t8349 - F::cast_from(4.0_f64) * t2296 * t8376 + F::cast_from(24.0_f64) * t23708 * t23709 - F::cast_from(36.0_f64) * t8345 * t2302 * t2315 + F::cast_from(6.0_f64) * t2301 * t23715 + F::cast_from(8.0_f64) * t2301 * t979 * t8376 - t974 * (t23732 + t23745 + t23758 + t23771);
    t23775
}
