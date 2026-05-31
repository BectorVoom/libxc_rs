//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 380/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk380<F: Float>(t1101: F, t1108: F, t14: F, t1747: F, t1751: F, t1759: F, t344: F, t659: F, t257: F, t670: F, t623: F, t78: F) -> (F, F, F, F) {
    let t1762 = -t1747 * t1101 / F::cast_from(18.0_f64) - t1751 * t344 / F::cast_from(6.0_f64) + t659 * t1108 / F::cast_from(9.0_f64) + t14 * t1759 / F::cast_from(2.0_f64);
    let t1763 = t257 * t1762;
    let t1767 = t670 * t670;
    let t1772 = t78 * t623;
    (t1762, t1763, t1767, t1772)
}
