//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1150/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1150<F: Float>(t139390: F, t139410: F, t139413: F, t139416: F, t148545: F, t148551: F, t148555: F, t148559: F, t148563: F, t148567: F, t148571: F, t148573: F, t148578: F, t148580: F, t148583: F, t148587: F) -> F {
    let t148589 = -t139390 / F::cast_from(54.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t148545 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t139410 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t139413 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t139416 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t148551 - F::cast_from(4.0_f64) * t148555 - t148559 / F::cast_from(6.0_f64) + t148563 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t148567 + t148571 / F::cast_from(2.0_f64) - t148573 / F::cast_from(27.0_f64) + t148578 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t148580 - t148583 / F::cast_from(36.0_f64) - t148587 / F::cast_from(6.0_f64);
    t148589
}
