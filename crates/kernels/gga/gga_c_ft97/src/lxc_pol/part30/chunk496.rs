//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 496/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk496<F: Float>(t737: F, t754: F, t2344: F, t675: F, t251: F, t631: F, t7242: F, t898: F, t2371: F, t665: F) -> (F, F, F, F) {
    let t9787 = t737 * t754;
    let t9802 = t2344 * t675;
    let t9890 = F::cast_from(1.0_f64) / t251 / t631 / t898 / t675 / t7242 / F::cast_from(4.0_f64);
    let t9895 = t665 * t2371;
    (t9787, t9802, t9890, t9895)
}
