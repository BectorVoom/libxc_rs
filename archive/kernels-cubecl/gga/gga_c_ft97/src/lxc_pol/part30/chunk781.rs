//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 781/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk781<F: Float>(t33859: F, t852: F, t193: F, t6308: F, t33815: F, t33819: F, t33825: F, t33833: F, t33838: F, t33842: F, t33846: F, t33850: F, t33854: F, t33857: F) -> (F, F, F) {
    let t33860 = t852 * t33859;
    let t33862 = t6308 * t193 * t33860;
    let t33864 = t33815 / F::cast_from(2.0_f64) + t33819 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t33825 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t33833 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t33838 - t33842 / F::cast_from(6.0_f64) - t33846 - t33850 / F::cast_from(9.0_f64) - t33854 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t33857 + t33862 / F::cast_from(12.0_f64);
    (t33860, t33862, t33864)
}
