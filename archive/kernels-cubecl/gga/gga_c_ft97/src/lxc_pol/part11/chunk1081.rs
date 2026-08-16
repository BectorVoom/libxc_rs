//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1081/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1081<F: Float>(t10020: F, t1882: F, t9840: F, t10131: F, t10002: F, t10024: F, t10029: F, t10034: F, t2459: F, t2469: F, t2568: F, t2569: F, t2574: F, t2594: F, t265: F, t41753: F, t41794: F, t42455: F, t42469: F, t446: F, t729: F, t773: F, t9572: F, t9578: F) -> F {
    let t42474 = t1882 * t10020;
    let t42476 = t1882 * t9840;
    let t42482 = t1882 * t10131;
    let t42488 = -F::cast_from(4.0_f64) * t446 * t729 * t2568 * t2569 * t2459 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t42455 - F::cast_from(8.0_f64) * t446 * t2574 * t2469 * t10029 - F::cast_from(8.0_f64) * t446 * t729 * t10002 * t10034 - F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t446 * t10024 * t773 * t9572 - F::cast_from(80.0_f64) / F::cast_from(243.0_f64) * t446 * t42469 * t265 * t41753 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t42474 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t42476 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t446 * t2594 * t773 * t9578 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t42482 - t446 * t729 * t265 * t41794 / F::cast_from(3.0_f64);
    t42488
}
