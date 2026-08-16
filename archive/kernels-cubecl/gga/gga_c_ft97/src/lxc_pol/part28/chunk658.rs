//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 658/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk658<F: Float>(t452: F, t5617: F, t986: F, t23311: F, t23312: F, t23319: F, t23321: F, t23344: F, t23358: F, t23360: F, t26461: F, t26464: F, t26468: F, t26472: F, t26476: F, t26480: F, t28: F, t446: F, t89: F) -> F {
    let t26487 = t452 * t986 * t5617;
    let t26490 = -t23311 + t23312 / F::cast_from(9.0_f64) + t23319 / F::cast_from(9.0_f64) + t23321 / F::cast_from(9.0_f64) - t446 * t26461 / F::cast_from(3.0_f64) - t446 * t26464 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t26468 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t26472 + t89 * t28 * t26476 / F::cast_from(3.0_f64) - t446 * t26480 / F::cast_from(3.0_f64) - t23344 / F::cast_from(27.0_f64) + t23358 / F::cast_from(9.0_f64) + t23360 / F::cast_from(9.0_f64) - t446 * t26487 / F::cast_from(3.0_f64);
    t26490
}
