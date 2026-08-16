//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 499/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk499<F: Float>(t2: F, t7241: F, t1542: F, t17: F, t1554: F, t369: F, t631: F, t637: F, t7242: F, t96: F, t1786: F, t480: F) -> (F, F, F, F, F) {
    let t8270 = t7241 * t2;
    let t8281 = t1542 * t17;
    let t8326 = t1554 * t369;
    let t8345 = F::cast_from(1.0_f64) / t96 / t631 / t637 / t369 / t7242 / F::cast_from(4.0_f64);
    let t8372 = t1786 * t480;
    (t8270, t8281, t8326, t8345, t8372)
}
