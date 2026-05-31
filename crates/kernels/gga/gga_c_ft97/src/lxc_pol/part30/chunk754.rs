//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 754/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk754<F: Float>(t33339: F, t33458: F, t33474: F, t33344: F, t33349: F, t33455: F, t33463: F, t33467: F, t33471: F, t33479: F, t33483: F, t33487: F) -> (F, F, F, F) {
    let t33518 = t33339 / F::cast_from(18.0_f64);
    let t33522 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t33458;
    let t33526 = t33474 / F::cast_from(9.0_f64);
    let t33530 = t33518 + t33344 / F::cast_from(18.0_f64) + t33349 / F::cast_from(3.0_f64) - t33455 / F::cast_from(6.0_f64) - t33522 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t33463 - F::cast_from(2.0_f64) * t33467 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t33471 + t33526 + t33479 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t33483 - t33487 / F::cast_from(3.0_f64);
    (t33518, t33522, t33526, t33530)
}
