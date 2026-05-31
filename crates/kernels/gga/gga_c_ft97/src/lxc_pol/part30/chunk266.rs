//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 266/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk266<F: Float>(t3746: F, t683: F, t3051: F, t2401: F, t2402: F, t3738: F, t3741: F, t3744: F, t200: F, t680: F, t2379: F, t3733: F) -> (F, F, F, F, F) {
    let t3747 = t683 * t3746;
    let t3748 = t3051 * t3747;
    let t3750 = t2401 + t2402 / F::cast_from(9.0_f64) + t3738 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3741 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3744 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3748;
    let t3751 = t3750 * t200;
    let t3752 = t680 * t3751;
    let t3755 = t2379 * t3733;
    (t3748, t3750, t3751, t3752, t3755)
}
