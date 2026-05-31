//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 238/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk238<F: Float>(t1544: F, t376: F, t482: F, t89: F, t102: F, t486: F) -> (F, F, F) {
    let t1832 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1544;
    let t1848 = t89 * t376 * t482;
    let t1851 = F::cast_from(1.0_f64) / t486 / t102;
    (t1832, t1848, t1851)
}
