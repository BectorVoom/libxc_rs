//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 640/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk640<F: Float>(t6837: F, t729: F, t773: F, t242: F, t27987: F, t1901: F, t24731: F, t24733: F, t24735: F, t28286: F, t28289: F, t28291: F, t28295: F, t28302: F, t28305: F, t28309: F, t28312: F, t446: F) -> F {
    let t28319 = t729 * t773 * t6837;
    let t28322 = t242 * t27987;
    let t28325 = t446 * t28286 / F::cast_from(3.0_f64) + t28289 / F::cast_from(27.0_f64) + t1901 * t28291 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t28295 - F::cast_from(2.0_f64) * t1901 * t28302 + t1901 * t28305 / F::cast_from(9.0_f64) + t1901 * t28309 / F::cast_from(9.0_f64) + t1901 * t28312 / F::cast_from(9.0_f64) + t24731 / F::cast_from(9.0_f64) + t24733 / F::cast_from(9.0_f64) + t24735 / F::cast_from(9.0_f64) - t446 * t28319 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t28322;
    t28325
}
