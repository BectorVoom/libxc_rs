//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 763/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk763<F: Float>(t2574: F, t7440: F, t773: F, t33605: F, t33609: F, t33613: F, t33617: F, t33622: F, t33626: F, t33630: F, t33632: F, t33636: F, t33638: F, t33642: F, t446: F) -> (F, F) {
    let t33646 = t2574 * t773 * t7440;
    let t33649 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t33605 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t33609 - t446 * t33613 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) * t446 * t33617 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t33622 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t33626 - t33630 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t33632 + t33636 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t33638 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t33642 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t33646;
    (t33646, t33649)
}
