//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1066/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1066<F: Float>(t151230: F, t151247: F, t151264: F, t151278: F, t151296: F, t151312: F, t151327: F, t151344: F, t1403: F, t141524: F, t141527: F, t141543: F, t1427: F, t1454: F, t151139: F, t151188: F, t151200: F, t151212: F, t193: F, t2: F, t258: F, t26: F, t27894: F, t27906: F, t27908: F, t33568: F, t35276: F, t4: F, t5996: F, t6840: F, t7437: F, t7487: F) -> (F, F) {
    let t151347 = t151230 + t151247 + t151264 + t151278 + t151296 + t151312 + t151327 + t151344;
    let t151350 = (t151139 + t151188) * t2 * t4 * t26 * t1427 / F::cast_from(6.0_f64) + t7437 * t27908 / F::cast_from(6.0_f64) - t141524 / F::cast_from(18.0_f64) - t141527 / F::cast_from(18.0_f64) + t151200 / F::cast_from(9.0_f64) + t27894 * t7487 / F::cast_from(6.0_f64) + t5996 * t35276 / F::cast_from(3.0_f64) + t1403 * t193 * t27906 * t1454 / F::cast_from(3.0_f64) + t33568 * t6840 / F::cast_from(6.0_f64) + t151212 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) * t151347 * t258 + t141543;
    (t151347, t151350)
}
