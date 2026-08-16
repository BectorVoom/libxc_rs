//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 978/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk978<F: Float>(t2106: F, t8282: F, t1775: F, t9211: F, t9230: F, t11755: F, t11761: F, t12796: F, t17338: F, t1985: F, t1986: F, t2: F, t2075: F, t2097: F, t2112: F, t24: F, t38556: F, t38562: F, t38572: F, t38588: F, t39769: F, t40323: F, t40327: F, t40335: F, t40337: F, t462: F, t558: F, t582: F, t9007: F, t9016: F, t92: F) -> F {
    let t40357 = t8282 * t2106;
    let t40359 = t1775 * t9211;
    let t40361 = t1775 * t9230;
    let t40367 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t11755 * t12796 * t40323 - F::cast_from(8.0_f64) * t11761 * t17338 * t40327 * t558 + F::cast_from(8.0_f64) * t462 * t582 * t38562 + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t40335 - F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t462 * t40337 * t38572 - F::cast_from(36.0_f64) * t462 * t9016 * t2 * t1986 * t2075 - F::cast_from(8.0_f64) * t462 * t2097 * t38588 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t2097 * t38556 + F::cast_from(8.0_f64) * t462 * t1985 * t2 * t9007 * t558 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t40357 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t40359 - F::cast_from(8.0_f64) * t40361 + F::cast_from(6.0_f64) * t92 * t24 * t2112 * t39769;
    t40367
}
